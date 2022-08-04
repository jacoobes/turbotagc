const { compile } = require('../pkg/turbotagc')

compile(String.raw`
    start 
      pooba > c | d | e | end a dsf dsa {{ a }}
    end
`);


/**
 *  Expr
 *
 *  prim = var | bool | whitespace | template
 *  pipeable = prim '>' (prim '|')+
 *  fillin = '{{' expr+ '}}'
 *  expr = prim | pipeable | fillin
 *
 *
 *
 *
 *
 *
 *
 */