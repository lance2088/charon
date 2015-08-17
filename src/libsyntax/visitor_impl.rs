#![allow(dead_code)]

use ast::*;
use visit::{Visitor};

/*impl<T> Visitor<T> for CodeGenVisitor where T: std::fmt::Display {
    fn visit(&self, t: T) {
        println!("{}", t);
    }
}*/

/*struct CodeGenVisitor;
struct ExpVisitor{
    result : i32
}

impl<'a> Visitor<&'a NumExpression> for ExpVisitor {
    fn visit(&mut self, n: &'a NumExpression) {
        self.result = n.value;
    }
}

impl<'a> Visitor<&'a AddExpression> for ExpVisitor {
    fn visit(&mut self, add: &'a AddExpression) {
        let b1 = *(add.e1);
        b1.accept(self);
        let r1 = self.result;
        add.e2.accept(self);
        let r2 = self.result;
        self.result = r1 + r2;
    }
}*/

struct ExpEvaluator{
    value : i32
}

impl ExpEvaluator{
    fn get_value(&mut self, expr : & Expr)->i32{
        match expr{
            &Expr::NumExpr(value) => value,
            _ => 1
        }
    }
}

impl<'a> Visitor<'a> for ExpEvaluator{
    fn visit_expr(&mut self, expr: &'a Expr){
        match expr{
            &Expr::AddExpr(ref left, ref right) => {
                let v1 = self.get_value(left);
                let v2 = self.get_value(right);
                self.value = v1 + v2;
            },
            _ => {}
        }
    }
}

struct TypeChecker{
    ty : LuaType
}

impl<'a> Visitor<'a> for TypeChecker{
    fn visit_expr(&mut self, expr: &'a Expr){
        match expr{
            &Expr::AddExpr(ref left, ref right) => {
                   
            },
            _ => {}
        }
    }
    
    /*fn visit_numexpr(&mut self, expr: &'a NumExpr){
        self.ty = LuaType::LNumber(expr.value);
    }*/
}

struct PrettyPrintVisitor;

impl<'a> Visitor<'a> for PrettyPrintVisitor{
    fn visit_expr(&mut self, expr:&'a Expr){
        match expr{
            &Expr::AddExpr(ref left, ref right) => {
                self.visit_expr(left);
                println!(" Plus ");
                self.visit_expr(right);
            },
            &Expr::NumExpr(value) => {
                println!(" Num({}) ", value);
            },
            &Expr::IdentExpr(ref value) => {
                println!(" Ident({}) ", value);
            }
        }
    }
}