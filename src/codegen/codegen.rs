use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::values::BasicValueEnum;
use std::fs::File;
use std::io::Write;
use crate::ast::{Expr, BinOp, Statement};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("venti");
        let builder = context.create_builder();
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        
        // Declare printf function
        let i32_type = context.i32_type();
        let i8ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[i8ptr_type.into()], true);
        module.add_function("printf", printf_type, None);

        CodeGen {
            context,
            builder,
            module,
            execution_engine,
        }
    }

    pub fn compile(&self, statements: Vec<Statement>) {
        for statement in statements {
            self.compile_statement(statement);
        }

        // Print the generated LLVM IR to a file
        let ir = self.module.print_to_string().to_string();
        let mut file = File::create("output.ll").expect("Could not create file");
        file.write_all(ir.as_bytes()).expect("Could not write to file");
    }

    fn compile_statement(&self, statement: Statement) {
        match statement {
            Statement::VariableDeclaration { identifier, value } => {
                let value = self.compile_expr(value);
                let global = self.module.add_global(value.get_type(), None, &identifier);
                global.set_initializer(&value);
            }
            Statement::Print(expr) => {
                let value = self.compile_expr(expr);
                let printf = self.module.get_function("printf").expect("Expected 'printf' function");
                self.builder.build_call(printf, &[value.into()], "printf_call");
            }
        }
    }

    fn compile_expr(&self, expr: Expr) -> BasicValueEnum<'ctx> {
        match expr {
            Expr::Number(n) => self.context.i64_type().const_int(n as u64, false).into(),
            Expr::String(s) => self.builder.build_global_string_ptr(&s, "str").as_pointer_value().into(),
            Expr::Identifier(id) => self.module.get_global(&id).unwrap().as_pointer_value().into(),
            Expr::BinaryOp(left, op, right) => {
                let left = self.compile_expr(*left).into_int_value();
                let right = self.compile_expr(*right).into_int_value();
                match op {
                    BinOp::Add => self.builder.build_int_add(left, right, "tmpadd").into(),
                    BinOp::Subtract => self.builder.build_int_sub(left, right, "tmpsub").into(),
                    BinOp::Multiply => self.builder.build_int_mul(left, right, "tmpmul").into(),
                    BinOp::Divide => self.builder.build_int_signed_div(left, right, "tmpdiv").into(),
                }
            }
            _ => panic!("Unsupported expression"),
        }
    }
}
