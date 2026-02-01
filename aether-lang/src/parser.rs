use chumsky::prelude::*;
use crate::ast::*;
use crate::lexer::Token;

pub fn parser() -> impl Parser<Token, Vec<TopLevel>, Error = Simple<Token>> {
    let ident = select! { Token::Ident(i) => i };
    
    let string_lit = select! { Token::StringLit(s) => s };
        
    // Type Parser (Simplified)
    // Vector<f32, 1536> or String
    let type_parser = recursive(|ty| {
        let primitive = ident.map(Type::Primitive);
        
        let vector = ident.filter(|n| n == "Vector")
            .ignore_then(just(Token::BracketOpen).or(just(Token::ParenOpen).ignored())) // Hack for demo < vs ( 
            .ignore_then(ident) // f32
            .then_ignore(just(Token::Comma))
            .then(select! { Token::IntLit(x) => x as usize }) // 1536
            .then_ignore(just(Token::BracketClose).or(just(Token::ParenClose).ignored()))
            .map(|(subtype, size)| Type::Vector(subtype, size));

        vector.or(primitive)
    });

    // Expressions
    let expr = recursive(|expr| {
        let literal = select! {
            Token::StringLit(s) => Expr::Literal(Literal::String(s)),
            Token::IntLit(i) => Expr::Literal(Literal::Int(i)),
        };
        
        let variable = ident.map(Expr::Variable);
        
        let atom = literal.or(variable)
            .delimited_by(just(Token::ParenOpen), just(Token::ParenClose))
            .or(literal)
            .or(variable);

        // Method calls: model.predict(input)
        // Note: simplified left-recursion handling for demo
        atom.then(just(Token::Dot).ignore_then(ident).then(
            expr.clone().separated_by(just(Token::Comma))
            .delimited_by(just(Token::ParenOpen), just(Token::ParenClose))
        ).repeated())
        .foldl(|lhs, (method, args)| {
             Expr::MethodCall(Box::new(lhs), method, args)
        })
    });

    // Statements
    let stmt = recursive(|stmt| {
        let let_stmt = just(Token::Let)
            .ignore_then(ident)
            .then_ignore(just(Token::Equals))
            .then(expr.clone())
            .then_ignore(just(Token::Semi))
            .map(|(name, e)| Statement::Let(name, e));

        let return_stmt = just(Token::Return)
            .ignore_then(expr.clone())
            .then_ignore(just(Token::Semi))
            .map(Statement::Return);

        let block = stmt.repeated().delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
            .map(|stmts| Block { statements: stmts });
            
        let thinking_stmt = just(Token::Thinking)
            .ignore_then(string_lit)
            .then(block.clone())
            .map(|(desc, blk)| Statement::Thinking(desc, blk));

        let_stmt.or(return_stmt).or(thinking_stmt)
            .or(expr.clone().then_ignore(just(Token::Semi)).map(Statement::Expr))
    });

    let block = stmt.repeated().delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
        .map(|stmts| Block { statements: stmts });

    // Function
    let function = just(Token::Fn)
        .ignore_then(ident)
        .then(
            ident.then_ignore(just(Token::Colon)).then(type_parser.clone())
                .separated_by(just(Token::Comma))
                .delimited_by(just(Token::ParenOpen), just(Token::ParenClose))
        )
        .then(just(Token::Arrow).ignore_then(type_parser.clone()).or_not())
        .then(block)
        .map(|(((name, args), ret), body)| FunctionDef {
            name,
            args,
            return_type: ret,
            body
        });

    // Agent
    let decorator = just(Token::ComputeDecorator)
        .ignore_then(
            ident.then_ignore(just(Token::Equals)).then(string_lit)
            .separated_by(just(Token::Comma))
            .delimited_by(just(Token::ParenOpen), just(Token::ParenClose))
        )
        .map(|args| Decorator { name: "compute".to_string(), arguments: args });

    let state_block = just(Token::State)
        .ignore_then(
             ident.then_ignore(just(Token::Colon)).then(type_parser.clone())
             .separated_by(just(Token::Comma))
             .delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
        )
        .map(|fields| StateBlock { 
            fields: fields.into_iter().map(|(n, t)| StateField { name: n, ty: t }).collect() 
        });

    let agent = decorator.repeated()
        .then_ignore(just(Token::Agent))
        .then(ident)
        .then(just(Token::BraceOpen))
        .then(state_block.or_not())
        .then(function.repeated())
        .then_ignore(just(Token::BraceClose))
        .map(|((((decorators, name), _), state), functions)| TopLevel::Agent(AgentDef {
            name,
            decorators,
            state,
            functions
        }));

    agent.repeated().then_ignore(end())
}
