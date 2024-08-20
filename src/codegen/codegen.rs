use crate::errors::VentiError;
use crate::venti_parser::ast::{BinOp, Expr, Statement};
use async_std::task;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, IntType};
use inkwell::values::{BasicValueEnum, FloatValue, IntValue};
use inkwell::OptimizationLevel;
use std::fs::File;
use std::io::Write;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

/*
The CodeGen struct is responsible for generating LLVM IR code from a parsed abstract syntax tree (AST). It uses the inkwell library to interact with the LLVM backend, allowing the program to be compiled and executed dynamically.

Fields:
*    context: A reference to the LLVM Context, which manages the memory and lifetime of LLVM objects.
*    builder: A Builder used to generate LLVM instructions.
*    module: A Module that contains the generated code.
*    execution_engine: An ExecutionEngine that allows the JIT (Just-In-Time) compilation and execution of the generated code.
*    Methods: new(context: &'ctx Context) -> Self
*    Creates a new CodeGen instance. This method initializes the LLVM Module, Builder, and ExecutionEngine. It also declares the printf function for use in generated code.

Parameters:

*   context: A reference to the LLVM context.
Returns:

*   A new instance of CodeGen.
*   compile(&self, statements: Vec<Statement>) -> Result<(), VentiError>
*    Compiles a vector of Statement AST nodes into LLVM IR. The generated code is then written to a file named output.ll.

Parameters:

*   statements: A vector of Statement nodes representing the program to be compiled.
Returns:
Ok(()) on success, or an error (VentiError) if something goes wrong during compilation.
compile_statement(&self, statement: Statement) -> Result<(), VentiError>
Compiles a single Statement into LLVM IR.

Parameters:

statement: The Statement node to be compiled.

Returns:
Ok(()) on success, or an error (VentiError) if the statement cannot be compiled.
compile_async_function(&self, identifier: String, body: Vec<Statement>) -> Result<(), VentiError>
Compiles an asynchronous function by generating LLVM code for the function body and appending it to the module.

Parameters:

identifier: The name of the async function.
body: A vector of Statement nodes representing the function body.

Returns:
Ok(()) on success, or an error (VentiError) if the function cannot be compiled.
compile_expr(&self, expr: Expr) -> Result<BasicValueEnum<'ctx>, VentiError>
Compiles an expression (Expr) into LLVM IR, returning the generated value.

Parameters:

expr: The expression node to be compiled.

Returns:

A BasicValueEnum containing the compiled value, or an error (VentiError) if the expression cannot be compiled.
async_task(&self, value: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx>
A placeholder function for handling asynchronous tasks. In an actual implementation, this function would handle the creation and execution of asynchronous tasks.

Parameters:

value: The value to be handled asynchronously.

Returns:

The value as a BasicValueEnum. (In the current implementation, this function simply returns the input value.)
*/

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("venti");
        let builder = context.create_builder();
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        // Declare printf function
        let i32_type = context.i32_type();
        let i8ptr_type = context
            .i8_type()
            .ptr_type(inkwell::AddressSpace::Generic.into());
        let printf_type = i32_type.fn_type(&[i8ptr_type.into()], true);
        module.add_function("printf", printf_type, None);

        CodeGen {
            context,
            builder,
            module,
            execution_engine,
        }
    }

    pub fn compile(&self, statements: Vec<Statement>) -> Result<(), VentiError> {
        for statement in statements {
            self.compile_statement(statement)?;
        }

        // Print the generated LLVM IR to a file
        let ir = self.module.print_to_string().to_string();
        let mut file = File::create("output.ll").map_err(|e| VentiError::IOError(e.to_string()))?;
        file.write_all(ir.as_bytes())
            .map_err(|e| VentiError::IOError(e.to_string()))?;
        Ok(())
    }

    fn compile_statement(&self, statement: Statement) -> Result<(), VentiError> {
        match statement {
            Statement::VariableDeclaration { identifier, value } => {
                let value = self.compile_expr(value)?;
                let global = self.module.add_global(value.get_type(), None, &identifier);
                global.set_initializer(&value);
                Ok(())
            }
            Statement::FunctionCall { identifier, args } => {
                let function = self.module.get_function(&identifier).ok_or_else(|| {
                    VentiError::CodegenError(format!("Undefined function '{}'", identifier))
                })?;
                let compiled_args = args
                    .into_iter()
                    .map(|arg| self.compile_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                self.builder
                    .build_call(function, &compiled_args, "call_func");
                Ok(())
            }
            Statement::Print(expr) => {
                let value = self.compile_expr(expr)?;
                let printf = self.module.get_function("printf").ok_or_else(|| {
                    VentiError::CodegenError("Expected 'printf' function".to_string())
                })?;
                self.builder
                    .build_call(printf, &[value.into()], "printf_call");
                Ok(())
            }
            Statement::AsyncFunction { identifier, body } => {
                self.compile_async_function(identifier, body)
            }
        }
    }

    fn compile_async_function(
        &self,
        identifier: String,
        body: Vec<Statement>,
    ) -> Result<(), VentiError> {
        let func_type = self.context.void_type().fn_type(&[], false);
        let function = self.module.add_function(&identifier, func_type, None);
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        for statement in body {
            self.compile_statement(statement)?;
        }

        self.builder.build_return(None);
        Ok(())
    }

    fn compile_expr(&self, expr: Expr) -> Result<BasicValueEnum<'ctx>, VentiError> {
        match expr {
            Expr::Number(n) => Ok(self.context.i64_type().const_int(n as u64, false).into()),
            Expr::Float(f) => Ok(self.context.f64_type().const_float(f).into()),
            Expr::Boolean(b) => Ok(self.context.i1_type().const_int(b as u64, false).into()),
            Expr::String(s) => Ok(self
                .builder
                .build_global_string_ptr(&s, "str")
                .as_pointer_value()
                .into()),
            Expr::Identifier(id) => {
                let global = self.module.get_global(&id).ok_or_else(|| {
                    VentiError::CodegenError(format!("Undefined variable '{}'", id))
                })?;
                Ok(global.as_pointer_value().into())
            }
            Expr::BinaryOp(left, op, right) => {
                let left = self.compile_expr(*left)?.into_int_value();
                let right = self.compile_expr(*right)?.into_int_value();
                let result = match op {
                    BinOp::Add => self.builder.build_int_add(left, right, "tmpadd"),
                    BinOp::Subtract => self.builder.build_int_sub(left, right, "tmpsub"),
                    BinOp::Multiply => self.builder.build_int_mul(left, right, "tmpmul"),
                    BinOp::Divide => self.builder.build_int_signed_div(left, right, "tmpdiv"),
                };
                Ok(result.into())
            }
            Expr::Array(elements) => {
                let array_type = self.context.i32_type().array_type(elements.len() as u32);
                let array_alloca = self.builder.build_alloca(array_type, "array");
                for (i, element) in elements.into_iter().enumerate() {
                    let value = self.compile_expr(element)?.into_int_value();
                    let index = self.context.i32_type().const_int(i as u64, false);
                    let ptr = unsafe {
                        self.builder
                            .build_gep(array_alloca, &[index.into()], "element_ptr", false)
                    };
                    self.builder.build_store(ptr, value);
                }
                Ok(array_alloca.into())
            }
            Expr::Await(inner_expr) => {
                let inner_value = self.compile_expr(*inner_expr)?;
                let async_task = self.async_task(inner_value);
                Ok(async_task.into())
            }
            _ => Err(VentiError::CodegenError(
                "Unsupported expression".to_string(),
            )),
        }
    }

    fn async_task(&self, value: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        // Placeholder: In actual implementation, handle async task creation
        value
    }
}
