const { compile } = require('../pkg/turbotagc')

compile(String.raw`
    {{ lorem \s\s ipsum }} [ a > c | d ]
`);


/**
 *  Expr
 *
 *
 *  parenthesis -> prim
 *  prim -> prim
 *
 *
 *
 *
 *
 */