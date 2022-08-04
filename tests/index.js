const { compile } = require('../pkg/turbotagc')

compile(String.raw`
    def teo = dsf dsf d f end
    start   
      ($p)ooba > c | d | e | end a dsf dsa {{ a }}
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