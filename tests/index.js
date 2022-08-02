const { compile } = require('../pkg/turbotagc')

compile(`{{
            [ 
            asd > a | b 
            ]
        }}
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