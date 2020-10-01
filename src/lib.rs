//  Copyright (c) 2020 Christopher Taylor
//
//  SPDX-License-Identifier: BSL-1.0
//  Distributed under the Boost Software License, Version 1.0. (See accompanying
//  file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//
#[macro_use]
mod armd4ocl {

#[macro_export]
macro_rules! opencl_str {

    // catches kernel block's terminal case
    //
    () => {
        concat!("");
    };

    ($a:tt;) => {
        concat!(stringify!($a), ";");
    };
    ($a:tt; $($tail:tt)*) => {
        concat!(stringify!($a), ";\n", opencl_str!($($tail)*));
    };

    // function definition
    //
    // https://rreverser.com/writing-complex-macros-in-rust/
    //
    // this captures function arguments
    //
    (@fargs) => {
        concat!("");
    };
    (@fargs $tok:tt $($tail:tt)* ) => {
        concat!( stringify!($tok), " ", opencl_str!(@fargs $($tail)*) );
    };
    (@fargs , $($tail:tt)* ) => {
        concat!( ",", opencl_str!(@fargs $($tail)*) );
    };

    // function definition blocks require this, macro compiler
    // does not match on $bk:block
    //
    (@fblock) => {
        concat!("");
    };
    (@fblock $tok:tt $($bk:tt)* ) => {
        concat!(stringify!($tok), opencl_str!(@fblock $($bk)*));
    };

    // function/kernel definition
    //
    (kernel $a:tt $b:tt ( $($args:tt)* ) { $($bk:tt)* } $($tail:tt)*) => {
        concat!(
            "kernel ", stringify!($a), " ", stringify!($b), " ( ",
                opencl_str!( @fargs $($args)* ),
            " ) {\n",
                opencl_str!( @fblock $($bk)* ),
            "\n}\n",
            opencl_str!($($tail)*)
        );
    };

    // function call
    //
    ($func:tt ( $($args:tt)* ); $($tail:tt)* ) => {
        concat!(
            stringify!($func),
            "(",
                opencl_str!( @fargs $($args)* ),
            ")",
            opencl_str!($($tail)*)
        );
    };

    // unary arithmetic operators
    //
    (++$a:tt) => {
        concat!("++", stringify!($a));
    };    
    (++$a:tt;) => {
        concat!("++", stringify!($a), ";");
    };
    (++$a:tt; $($tail:tt)*) => {
        concat!("++", stringify!($a), ";\n", opencl_str!($($tail)*));
    };

    ($a:ident++) => {
        concat!(stringify!($a), "++");
    };      
    ($a:tt++;) => {
        concat!(stringify!($a), "++;");
    };      
    ($a:tt++; $($tail:tt)*) => {
        concat!(stringify!($a), "++;\n", opencl_str!($($tail)*));
    };

    (--$a:ident) => {
        concat!("--", stringify!($a));
    };
    (--$a:tt;) => {
        concat!("--", stringify!($a), ";");
    };
    (--$a:tt; $($tail:tt)*) => {
        concat!("--", stringify!($a), ";\n", opencl_str!($($tail)*));
    };

    ($a:tt--) => {
        concat!(stringify!($a), "--");
    };      
    ($a:tt--;) => {
        concat!(stringify!($a), "--;");
    };      
    ($a:tt--; $($tail:tt)*) => {
        concat!(stringify!($a), "--;\n", opencl_str!($($tail)*));
    };

    // unary operator
    (*$a:tt;) => {
        concat!("*", stringify!($a), ";");
    };
    (*$a:tt; $($tail:tt)*) => {
        concat!("*", stringify!($a), ";\n", opencl_str!($($tail)*));
    };    
    (&$a:tt;) => {
        concat!("&", stringify!($a), ";");
    };
    (&$a:tt; $($tail:tt)*) => {
        concat!("&", stringify!($a), ";\n", opencl_str!($($tail)*));
    };

    // arithmetic operators
    //
    ( ($($e:tt)*) ) => {
        concat!("(", opencl_str!($($e)*), ")");
    };

    ($a:tt + $b:tt) => {
        concat!(stringify!($a), " + ", stringify!($b));
    };
    ($a:tt + $($b:tt)*) => {
        concat!(stringify!($a), " + ", opencl_str!($($b)*));
    };    
    ($a:tt + $b:tt;) => {
        concat!(stringify!($a), " + ", stringify!($b), ";");
    };    
    ($a:tt + $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " + ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };    

    ($a:tt - $b:tt) => {
        concat!(stringify!($a), "-", stringify!($b));
    };
    ($a:tt - $b:tt;) => {
        concat!(stringify!($a), " - ", stringify!($b), ";");
    };    
    ($a:tt - $($b:tt)*) => {
        concat!(stringify!($a), " - ", opencl_str!($($b)*));
    };        
    ($a:tt - $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " - ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };        
    
    ($a:tt * $b:tt) => {
        concat!(stringify!($a), "*", stringify!($b));
    };
    ($a:tt * $b:tt;) => {
        concat!(stringify!($a), " * ", stringify!($b), ";");
    };
    ($a:tt * $($b:tt)*) => {
        concat!(stringify!($a), " * ", opencl_str!($($b)*));
    };            
    ($a:tt * $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " * ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };

    ($a:tt / $b:tt) => {
        concat!(stringify!($a), "/", stringify!($b));
    };
    ($a:tt / $b:tt;) => {
        concat!(stringify!($a), " / ", stringify!($b;), ";");
    };    
    ($a:tt / $($b:tt)*) => {
        concat!(stringify!($a), " / ", opencl_str!($($b)*));
    };
    ($a:tt / $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " / ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };

    ($a:tt % $b:tt) => {
        concat!(stringify!($a), "%", stringify!($b));
    };
    ($a:tt % $b:tt;) => {
        concat!(stringify!($a), " % ", stringify!($b;), ";");
    };    
    ($a:tt % $($b:tt)*) => {
        concat!(stringify!($a), " % ", opencl_str!($($b)*));
    };
    ($a:tt % $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " % ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };

    // bitwise operators
    //
    ($a:tt & $b:tt) => {
        concat!(stringify!($a), "&", stringify!($b));
    };
    ($a:tt & $b:expr;) => {
        concat!(stringify!($a), " & ", opencl_str!($b;));
    };            
    ($a:tt & $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " & ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };                
    ($a:tt | $b:tt) => {
        concat!(stringify!($a), "|", stringify!($b));
    };
    ($a:tt | $b:expr;) => {
        concat!(stringify!($a), " | ", opencl_str!($b;));
    };            
    ($a:tt | $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " | ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };                    
    ($a:tt ^ $b:tt) => {
        concat!(stringify!($a), "^", stringify!($b));
    };
    ($a:tt ^ $b:expr;) => {
        concat!(stringify!($a), " ^ ", opencl_str!($b;));
    };            
    ($a:tt ^ $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " ^ ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };                    
    ($a:tt ~ $b:tt) => {
        concat!(stringify!($a), "~", stringify!($b));
    };
    ($a:tt ~ $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " ~ ", opencl_str!($b;));
    };            
    ($a:tt ~ $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " ~ ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };                

    // logical operators
    //
    ($a:tt && $b:tt) => {
        concat!(stringify!($a), "&&", stringify!($b));
    };
    ($a:tt && $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " && ", opencl_str!($b;), ";\n", opencl_str!($($tail)*) );
    };            
    ($a:tt && $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " && ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };                    
    ($a:tt || $b:tt) => {
        concat!(stringify!($a), "||", stringify!($b));
    };
    ($a:tt || $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " || ", opencl_str!($b;), ";\n", opencl_str!($($tail)*) );
    };            
    ($a:tt || $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " || ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };                    
    ($a:tt ^^ $b:tt) => {
        concat!(stringify!($a), "^^", stringify!($b));
    };
    ($a:tt ^^ $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " ^^ ", opencl_str!($b;), ";\n", opencl_str!($($tail)*) );
    };            
    ($a:tt ^^ $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " ^^ ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };                    

    // shift operators
    //
    ($a:tt >> $b:tt) => {
        concat!(stringify!($a), " >> ", stringify!($b));
    };    
    ($a:tt >> $b:tt;) => {
        concat!(stringify!($a), " >> ", stringify!($b), ";");
    };
    ($a:tt >> $b:tt; $($tail:tt)*) => {
        concat!(stringify!($a), " >> ", stringify!($b), ";\n", opencl_str!($($tail)*));
    };

    ($a:tt << $b:tt) => {
        concat!(stringify!($a), " << ", stringify!($b));
    };    
    ($a:tt << $b:tt;) => {
        concat!(stringify!($a), " << ", stringify!($b), ";");
    };
    ($a:tt << $b:tt; $($tail:tt)*) => {
        concat!(stringify!($a), " << ", stringify!($b), ";\n", opencl_str!($($tail)*));
    };       

    // unary logical operators
    //
    (!$a:tt) => {
        concat!("!", stringify!($a));
    };

    // equality operators
    //
    ($a:tt == $b:tt) => {
        concat!(stringify!($a), "==", stringify!($b));
    };
    ($a:tt != $b:tt) => {
        concat!(stringify!($a), "!=", stringify!($b));
    };

    // ternary operators
    //
    ($a:tt ? $b:tt : $c:tt) => {
        concat!(stringify!($a), "?", stringify!($b), ":", stringify($c));
    };
    ( select( $a:tt, $b:tt, $c:tt) ) => {
        concat!("select(", stringify!($a), stringify!($b), stringify!($c), ")");
    };

    // assignment operator
    //
    ([$a:expr]) => {
        stringify!($a);
    };
    ($a:tt [ $b:expr ] = $c:expr;) => {
        concat!(stringify!($a), "[", opencl_str!([$b]), "] = ", opencl_str!($c;));
    };   
    ($a:tt [ $b:expr ] = $c:expr; $($tail:tt)*) => {
        concat!(stringify!($a), "[", opencl_str!([$b]), "] = ", opencl_str!($c;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt = $b:expr;) => {
        concat!(stringify!($a), " = ", opencl_str!($b;));
    };   
    ($a:tt = $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " = ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt >>= $b:expr;) => {
        concat!(stringify!($a), " >>= ", opencl_str!($b;));
    };   
    ($a:tt >>= $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " >>= ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt <<= $b:expr;) => {
        concat!(stringify!($a), " <<= ", opencl_str!($b;));
    };   
    ($a:tt <<= $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " <<= ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt += $b:expr;) => {
        concat!(stringify!($a), " += ", opencl_str!($b;));
    };   
    ($a:tt += $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " += ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt -= $b:expr;) => {
        concat!(stringify!($a), " -= ", opencl_str!($b;));
    };   
    ($a:tt -= $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " -= ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt *= $b:expr;) => {
        concat!(stringify!($a), " *= ", opencl_str!($b;));
    };   
    ($a:tt *= $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " *= ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt /= $b:expr;) => {
        concat!(stringify!($a), " /= ", opencl_str!($b;));
    };   
    ($a:tt /= $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " /= ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt %= $b:expr;) => {
        concat!(stringify!($a), " %= ", opencl_str!($b;));
    };   
    ($a:tt %= $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " %= ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt &= $b:expr;) => {
        concat!(stringify!($a), " &= ", opencl_str!($b;));
    };   
    ($a:tt &= $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " &= ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt |= $b:expr;) => {
        concat!(stringify!($a), " |= ", opencl_str!($b;));
    };   
    ($a:tt |= $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " |= ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };
    ($a:tt ^= $b:expr;) => {
        concat!(stringify!($a), " ^= ", opencl_str!($b;));
    };   
    ($a:tt ^= $b:expr; $($tail:tt)*) => {
        concat!(stringify!($a), " ^= ", opencl_str!($b;), "\n", opencl_str!($($tail)*) );
    };

    // uint rules
    //
    (uint $a:ident;) => {
        concat!("uint ", stringify!($a), ";");
    };    
    (uint $a:ident; $($tail:tt)*) => {
        concat!("uint ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (uint $a:ident[ $inner:expr ]; $($tail:tt)*) => {
        concat!("uint ", stringify!($a), "[", stringify!($inner), "];\n", opencl_str!($($tail)*));
    };
    (uint $a:ident[ $inner:expr ]; $($tail:tt)*) => {
        concat!("uint ", stringify!($a), "[", stringify!($inner), "];\n", opencl_str!($($tail)*));
    };    

    (uint2 $a:ident = (uint2)($x:tt, $y:tt);) => {
        concat!("uint2 ", stringify!($a), " = (uint2)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ");");
    };
    (uint2 $a:ident = (uint2)($x:tt, $y:tt); $($tail:tt)*) => {
        concat!("uint2 ", stringify!($a), " = (uint2)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ");\n", opencl_str!($($tail)*));
    };
    (uint3 $a:ident = (uint3)($x:tt, $y:tt, $z:tt);) => {
        concat!("uint3 ", stringify!($a), " = (uint3)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ");");
    };
    (uint3 $a:ident = (uint3)($x:tt, $y:tt, $z:tt); $($tail:tt)*) => {
        concat!("uint3 ", stringify!($a), " = (uint3)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ");\n", opencl_str!($($tail)*));
    };
    (uint4 $a:ident = (uint4)($x:tt, $y:tt, $z:tt, $w:tt);) => {
        concat!("uint4 ", stringify!($a), " = (uint4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ");");
    };
    (uint4 $a:ident = (uint4)($x:tt, $y:tt, $z:tt, $w:tt); $($tail:tt)*) => {
        concat!("uint4 ", stringify!($a), " = (uint4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ");\n", opencl_str!($($tail)*));
    };
    (uint8 $i:ident = (uint8)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt);) => {
        concat!("uint8 ", stringify!($i), " = (uint8)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ");");
    };
    (uint8 $i:ident = (uint8)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt); $($tail:tt)*) => {
        concat!("uint8 ", stringify!($i), " = (uint8)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ");\n", opencl_str!($($tail)*));
    };    
    (uint16 $i:ident = (float16)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $j:tt, $k:tt, $l:tt, $m:tt);) => {
        concat!("int16 ", stringify!($i), " = (int16)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ", ", opencl_str!([$e]), ", ", opencl_str!([$f]), ", ", opencl_str!([$g]), ", ", opencl_str!([$h]), ", ", opencl_str!([$j]), ", ", opencl_str!([$k]), ", ", opencl_str!([$l]), ", ", opencl_str!([$m]), ");");
    };
    (uint16 $i:ident = (float16)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $j:tt, $k:tt, $l:tt, $m:tt); $($tail:tt)*) => {
        concat!("int16 ", stringify!($i), " = (int16)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ", ", opencl_str!([$e]), ", ", opencl_str!([$f]), ", ", opencl_str!([$g]), ", ", opencl_str!([$h]), ", ", opencl_str!([$j]), ", ", opencl_str!([$k]), ", ", opencl_str!([$l]), ", ", opencl_str!([$m]), ");\n", opencl_str!($($tail)*));
    };

    (uint2 $a:ident;) => {
        concat!("uint2 ", stringify!($a), ";");
    };    
    (uint2 $a:ident; $($tail:tt)*) => {
        concat!("uint2 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (uint3 $a:ident;) => {
        concat!("uint3 ", stringify!($a), ";");
    };    
    (uint3 $a:ident; $($tail:tt)*) => {
        concat!("uint3 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };    
    (uint4 $a:ident;) => {
        concat!("uint4 ", stringify!($a), ";");
    };    
    (uint4 $a:ident; $($tail:tt)*) => {
        concat!("uint4 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (uint8 $a:ident;) => {
        concat!("uint8 ", stringify!($a), ";");
    };    
    (uint8 $a:ident; $($tail:tt)*) => {
        concat!("uint8 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (uint16 $a:ident;) => {
        concat!("uint16 ", stringify!($a), ";");
    };    
    (uint16 $a:ident; $($tail:tt)*) => {
        concat!("uint16 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };

    // int rules
    //
    (int $a:ident;) => {
        concat!("int ", stringify!($a), ";");
    };    
    (int $a:ident; $($tail:tt)*) => {
        concat!("int ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (int $a:ident = $($tail:tt)*) => {
        concat!("int ", stringify!($a), " = ", opencl_str!($($tail)*));
    };    
    (int $a:ident[ $inner:expr ]; $($tail:tt)*) => {
        concat!("int ", stringify!($a), "[", stringify!($inner), "];\n", opencl_str!($($tail)*));
    };
    (int $a:ident[ $inner:expr ]; $($tail:tt)*) => {
        concat!("int ", stringify!($a), "[", stringify!($inner), "];\n", opencl_str!($($tail)*));
    };
    (int $a:ident[ $inner:expr ]; $($tail:tt)*) => {
        concat!("int ", stringify!($a), "[", stringify!($inner), "];\n", opencl_str!($($tail)*));
    };        

    (int2 $a:ident = (int2)($x:tt, $y:tt);) => {
        concat!("int2 ", stringify!($a), " = (int2)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ");");
    };
    (int2 $a:ident = (int2)($x:tt, $y:tt); $($tail:tt)*) => {
        concat!("int2 ", stringify!($a), " = (int2)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ");\n", opencl_str!($($tail)*));
    };
    (int3 $a:ident = (int3)($x:tt, $y:tt, $z:tt);) => {
        concat!("int3 ", stringify!($a), " = (int3)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ");");
    };
    (int3 $a:ident = (int3)($x:tt, $y:tt, $z:tt); $($tail:tt)*) => {
        concat!("int3 ", stringify!($a), " = (int3)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ");\n", opencl_str!($($tail)*));
    };
    (int4 $a:ident = (int4)($x:tt, $y:tt, $z:tt, $w:tt);) => {
        concat!("int4 ", stringify!($a), " = (int4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ");");
    };
    (int4 $a:ident = (int4)($x:tt, $y:tt, $z:tt, $w:tt); $($tail:tt)*) => {
        concat!("int4 ", stringify!($a), " = (int4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ");\n", opencl_str!($($tail)*));
    };
    (int8 $i:ident = (int8)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt);) => {
        concat!("int8 ", stringify!($i), " = (int8)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ");");
    };
    (int8 $i:ident = (int8)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt); $($tail:tt)*) => {
        concat!("int8 ", stringify!($i), " = (int8)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ");\n", opencl_str!($($tail)*));
    };    
    (int16 $i:ident = (float16)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $j:tt, $k:tt, $l:tt, $m:tt);) => {
        concat!("int16 ", stringify!($i), " = (int16)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ", ", opencl_str!([$e]), ", ", opencl_str!([$f]), ", ", opencl_str!([$g]), ", ", opencl_str!([$h]), ", ", opencl_str!([$j]), ", ", opencl_str!([$k]), ", ", opencl_str!([$l]), ", ", opencl_str!([$m]), ");");
    };
    (int16 $i:ident = (float16)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $j:tt, $k:tt, $l:tt, $m:tt); $($tail:tt)*) => {
        concat!("int16 ", stringify!($i), " = (int16)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ", ", opencl_str!([$e]), ", ", opencl_str!([$f]), ", ", opencl_str!([$g]), ", ", opencl_str!([$h]), ", ", opencl_str!([$j]), ", ", opencl_str!([$k]), ", ", opencl_str!([$l]), ", ", opencl_str!([$m]), ");\n", opencl_str!($($tail)*));
    };


    (int2 $a:ident;) => {
        concat!("int2 ", stringify!($a), ";");
    };    
    (int2 $a:ident; $($tail:tt)*) => {
        concat!("int2 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (int2 $a:ident = $($tail:tt)*) => {
        concat!("int2 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };    
    (int3 $a:ident;) => {
        concat!("int3 ", stringify!($a), ";");
    };    
    (int3 $a:ident; $($tail:tt)*) => {
        concat!("int3 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };    
    (int3 $a:ident = $($tail:tt)*) => {
        concat!("int3 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };    
    (int4 $a:ident;) => {
        concat!("int4 ", stringify!($a), ";");
    };    
    (int4 $a:ident; $($tail:tt)*) => {
        concat!("int4 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (int4 $a:ident = $($tail:tt)*) => {
        concat!("int4 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };    
    (int8 $a:ident;) => {
        concat!("int8 ", stringify!($a), ";");
    };    
    (int8 $a:ident; $($tail:tt)*) => {
        concat!("int8 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (int8 $a:ident = $($tail:tt)*) => {
        concat!("int8 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };    
    (int16 $a:ident;) => {
        concat!("int16 ", stringify!($a), ";");
    };    
    (int16 $a:ident; $($tail:tt)*) => {
        concat!("int16 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };        
    (int16 $a:ident = $($tail:tt)*) => {
        concat!("int16 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };    

    // float rules
    //
    (float $a:ident;) => {
        concat!("float ", stringify!($a), ";");
    };    
    (float $a:ident; $($tail:tt)*) => {
        concat!("float ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (float $a:ident = $($tail:tt)*) => {
        concat!("float ", stringify!($a), " = ", opencl_str!($($tail)*));
    };
    (float $a:ident[ $inner:expr ]; $($tail:tt)*) => {
        concat!("float ", stringify!($a), "[", stringify!($inner), "];\n", opencl_str!($($tail)*));
    };
    (float $a:ident[ $inner:expr ]; $($tail:tt)*) => {
        concat!("float ", stringify!($a), "[", stringify!($inner), "];\n", opencl_str!($($tail)*));
    };       

    (float2 $a:ident = (float2)($x:tt, $y:tt);) => {
        concat!("float2 ", stringify!($a), " = (float4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ");");
    };
    (float2 $a:ident = (float2)($x:tt, $y:tt); $($tail:tt)*) => {
        concat!("float2 ", stringify!($a), " = (float4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ");\n", opencl_str!($($tail)*));
    };
    (float3 $a:ident = (float3)($x:tt, $y:tt, $z:tt);) => {
        concat!("float3 ", stringify!($a), " = (float3)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ");\n");
    };
    (float3 $a:ident = (float3)($x:tt, $y:tt, $z:tt); $($tail:tt)*) => {
        concat!("float3 ", stringify!($a), " = (float3)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ");\n", opencl_str!($($tail)*));
    };
    (float4 $a:ident = (float4)($x:tt, $y:tt, $z:tt, $w:tt);) => {
        concat!("float4 ", stringify!($a), " = (float4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ");");
    };
    (float4 $a:ident = (float4)($x:tt, $y:tt, $z:tt, $w:tt); $($tail:tt)*) => {
        concat!("float4 ", stringify!($a), " = (float4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ");\n", opencl_str!($($tail)*));
    };
    (float8 $i:ident = (float8)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt);) => {
        concat!("float8 ", stringify!($i), " = (float8)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ");");
    };
    (float8 $i:ident = (float8)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt); $($tail:tt)*) => {
        concat!("float8 ", stringify!($i), " = (float8)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ");\n", opencl_str!($($tail)*));
    };
    (float16 $i:ident = (float16)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $j:tt, $k:tt, $l:tt, $m:tt);) => {
        concat!("float16 ", stringify!($i), " = (float16)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ", ", opencl_str!([$e]), ", ", opencl_str!([$f]), ", ", opencl_str!([$g]), ", ", opencl_str!([$h]), ", ", opencl_str!([$j]), ", ", opencl_str!([$k]), ", ", opencl_str!([$l]), ", ", opencl_str!([$m]), ");");
    };
    (float16 $i:ident = (float16)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $j:tt, $k:tt, $l:tt, $m:tt); $($tail:tt)*) => {
        concat!("float16 ", stringify!($i), " = (float16)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ", ", opencl_str!([$e]), ", ", opencl_str!([$f]), ", ", opencl_str!([$g]), ", ", opencl_str!([$h]), ", ", opencl_str!([$j]), ", ", opencl_str!([$k]), ", ", opencl_str!([$l]), ", ", opencl_str!([$m]), ");\n", opencl_str!($($tail)*));
    };  


    (float2 $a:ident;) => {
        concat!("float2 ", stringify!($a), ";");
    };    
    (float2 $a:ident; $($tail:tt)*) => {
        concat!("float2 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (float2 $a:ident = $($tail:tt)*) => {
        concat!("float2 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };        
    (float3 $a:ident;) => {
        concat!("float3 ", stringify!($a), ";");
    };    
    (float3 $a:ident; $($tail:tt)*) => {
        concat!("float3 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };    
    (float3 $a:ident = $($tail:tt)*) => {
        concat!("float3 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };        
    (float4 $a:ident;) => {
        concat!("float4 ", stringify!($a), ";");
    };    
    (float4 $a:ident; $($tail:tt)*) => {
        concat!("float4 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (float4 $a:ident = $($tail:tt)*) => {
        concat!("float4 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };            
    (float8 $a:ident;) => {
        concat!("float8 ", stringify!($a), ";");
    };    
    (float8 $a:ident; $($tail:tt)*) => {
        concat!("float8 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (float8 $a:ident = $($tail:tt)*) => {
        concat!("float8 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };                
    (float16 $a:ident;) => {
        concat!("float16 ", stringify!($a), ";");
    };    
    (float16 $a:ident; $($tail:tt)*) => {
        concat!("float16 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    }; 
    (float16 $a:ident = $($tail:tt)*) => {
        concat!("float16 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };

    // double rules
    //
    (double $a:ident;) => {
        concat!("double ", stringify!($a), ";");
    };
    (double $a:ident; $($tail:tt)*) => {
        concat!("double ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (double $a:ident = $($tail:tt)*) => {
        concat!("double ", stringify!($a), " = ", opencl_str!($($tail)*));
    };
    (double $a:ident[ $inner:expr ];) => {
        concat!("double ", stringify!($a), "[", opencl_str!([$inner]), "];");
    };    
    (double $a:ident[ $inner:expr ]; $($tail:tt)*) => {
        concat!("double ", stringify!($a), "[", opencl_str!([$inner]), "];\n", opencl_str!($($tail)*));
    };
    (double $a:ident[ $inner:expr ]; $($tail:tt)*) => {
        concat!("double ", stringify!($a), "[", opencl_str!([$inner]), "];\n", opencl_str!($($tail)*));
    };

    (double2 $a:ident = (double2)($x:tt, $y:tt);) => {
        concat!("double2 ", stringify!($a), " = (double2)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ");");
    };
    (double2 $a:ident = (double2)($x:tt, $y:tt); $($tail:tt)*) => {
        concat!("double2 ", stringify!($a), " = (double2)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ");\n", opencl_str!($($tail)*));
    };
    (double3 $a:ident = (double3)($x:tt, $y:tt, $z:tt);) => {
        concat!("double3 ", stringify!($a), " = (double3)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ");");
    };
    (double3 $a:ident = (double3)($x:tt, $y:tt, $z:tt); $($tail:tt)*) => {
        concat!("double3 ", stringify!($a), " = (double3)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ");\n", opencl_str!($($tail)*));
    };
    (double4 $a:ident = (float4)($x:tt, $y:tt, $z:tt, $w:tt);) => {
        concat!("double4 ", stringify!($a), " = (double4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ");");
    };
    (double4 $a:ident = (float4)($x:tt, $y:tt, $z:tt, $w:tt); $($tail:tt)*) => {
        concat!("double4 ", stringify!($a), " = (double4)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ");\n", opencl_str!($($tail)*));
    };
    (double8 $i:ident = (double8)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt);) => {
        concat!("double8 ", stringify!($i), " = (double8)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ");");
    };
    (double8 $i:ident = (double8)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt); $($tail:tt)*) => {
        concat!("double8 ", stringify!($i), " = (double8)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ");\n", opencl_str!($($tail)*));
    };  
    (double16 $i:ident = (double16)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $j:tt, $k:tt, $l:tt, $m:tt);) => {
        concat!("double16 ", stringify!($i), " = (double16)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ", ", opencl_str!([$e]), ", ", opencl_str!([$f]), ", ", opencl_str!([$g]), ", ", opencl_str!([$h]), ", ", opencl_str!([$j]), ", ", opencl_str!([$k]), ", ", opencl_str!([$l]), ", ", opencl_str!([$m]), ");");
    };
    (double16 $i:ident = (double16)($x:tt, $y:tt, $z:tt, $w:tt, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $j:tt, $k:tt, $l:tt, $m:tt); $($tail:tt)*) => {
        concat!("double16 ", stringify!($i), " = (double16)(", opencl_str!([$x]), ", ", opencl_str!([$y]), ", ", opencl_str!([$z]), ", ", opencl_str!([$w]), ", ", opencl_str!([$a]), ", ", opencl_str!([$b]), ", ", opencl_str!([$c]), ", ", opencl_str!([$d]), ", ", opencl_str!([$e]), ", ", opencl_str!([$f]), ", ", opencl_str!([$g]), ", ", opencl_str!([$h]), ", ", opencl_str!([$j]), ", ", opencl_str!([$k]), ", ", opencl_str!([$l]), ", ", opencl_str!([$m]), ");\n", opencl_str!($($tail)*));
    };  
 
    (double2 $a:ident;) => {
        concat!("double2 ", stringify!($a), ";");
    };    
    (double2 $a:ident; $($tail:tt)*) => {
        concat!("double2 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (double2 $a:ident = $($tail:tt)*) => {
        concat!("double2 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };                
    (double3 $a:ident;) => {
        concat!("double3 ", stringify!($a), ";");
    };    
    (double3 $a:ident; $($tail:tt)*) => {
        concat!("double3 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };    
    (double3 $a:ident = $($tail:tt)*) => {
        concat!("double3 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };                
    (double4 $a:ident;) => {
        concat!("double4 ", stringify!($a), ";");
    };    
    (double4 $a:ident; $($tail:tt)*) => {
        concat!("double4 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (double4 $a:ident = $($tail:tt)*) => {
        concat!("double4 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };                
    (double8 $a:ident;) => {
        concat!("double8 ", stringify!($a), ";");
    };    
    (double8 $a:ident; $($tail:tt)*) => {
        concat!("double8 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };
    (double8 $a:ident = $($tail:tt)*) => {
        concat!("double8 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };                
    (double16 $a:ident;) => {
        concat!("double16 ", stringify!($a), ";");
    };    
    (double16 $a:ident; $($tail:tt)*) => {
        concat!("double16 ", stringify!($a), ";\n", opencl_str!($($tail)*));
    };     
    (double16 $a:ident = $($tail:tt)*) => {
        concat!("double16 ", stringify!($a), " = ", opencl_str!($($tail)*));
    };

    ($a:ident.x; $($tail:tt)*) => {
        concat!(stringify!($a), ".x;", opencl_str!($($tail)*));
    };
    ($a:ident.x = $($tail:tt)*) => {
        concat!(stringify!($a), ".x = ", opencl_str!($($tail)*));
    };
    ($a:ident.x += $($tail:tt)*) => {
        concat!(stringify!($a), ".x += ", opencl_str!($($tail)*));
    };
    ($a:ident.x -= $($tail:tt)*) => {
        concat!(stringify!($a), ".x -= ", opencl_str!($($tail)*));
    };
    ($a:ident.x *= $($tail:tt)*) => {
        concat!(stringify!($a), ".x *= ", opencl_str!($($tail)*));
    };
    ($a:ident.x /= $($tail:tt)*) => {
        concat!(stringify!($a), ".x /= ", opencl_str!($($tail)*));
    };            
    ($a:ident.x + $($tail:tt)*) => {
        concat!(stringify!($a), ".x + ", opencl_str!($($tail)*));
    };
    ($a:ident.x - $($tail:tt)*) => {
        concat!(stringify!($a), ".x - ", opencl_str!($($tail)*));
    };
    ($a:ident.x * $($tail:tt)*) => {
        concat!(stringify!($a), ".x * ", opencl_str!($($tail)*));
    };
    ($a:ident.x / $($tail:tt)*) => {
        concat!(stringify!($a), ".x / ", opencl_str!($($tail)*));
    };         
    ($a:ident.x >> $($tail:tt)*) => {
        concat!(stringify!($a), ".x >> ", opencl_str!($($tail)*));
    };     
    ($a:ident.x << $($tail:tt)*) => {
        concat!(stringify!($a), ".x << ", opencl_str!($($tail)*));
    };     
    ($a:ident.x >>= $($tail:tt)*) => {
        concat!(stringify!($a), ".x >>= ", opencl_str!($($tail)*));
    };     
    ($a:ident.x <<= $($tail:tt)*) => {
        concat!(stringify!($a), ".x <<= ", opencl_str!($($tail)*));
    };     

    ($a:ident.y; $($tail:tt)*) => {
        concat!(stringify!($a), ".y;", opencl_str!($($tail)*));
    };
    ($a:ident.y = $($tail:tt)*) => {
        concat!(stringify!($a), ".y = ", opencl_str!($($tail)*));
    };
    ($a:ident.y += $($tail:tt)*) => {
        concat!(stringify!($a), ".y += ", opencl_str!($($tail)*));
    };
    ($a:ident.y -= $($tail:tt)*) => {
        concat!(stringify!($a), ".y -= ", opencl_str!($($tail)*));
    };
    ($a:ident.y *= $($tail:tt)*) => {
        concat!(stringify!($a), ".y *= ", opencl_str!($($tail)*));
    };
    ($a:ident.y /= $($tail:tt)*) => {
        concat!(stringify!($a), ".y /= ", opencl_str!($($tail)*));
    };            
    ($a:ident.y + $($tail:tt)*) => {
        concat!(stringify!($a), ".y + ", opencl_str!($($tail)*));
    };
    ($a:ident.y - $($tail:tt)*) => {
        concat!(stringify!($a), ".y - ", opencl_str!($($tail)*));
    };
    ($a:ident.y * $($tail:tt)*) => {
        concat!(stringify!($a), ".y * ", opencl_str!($($tail)*));
    };
    ($a:ident.y / $($tail:tt)*) => {
        concat!(stringify!($a), ".y / ", opencl_str!($($tail)*));
    };            
    ($a:ident.y >> $($tail:tt)*) => {
        concat!(stringify!($a), ".y >> ", opencl_str!($($tail)*));
    };     
    ($a:ident.y << $($tail:tt)*) => {
        concat!(stringify!($a), ".y << ", opencl_str!($($tail)*));
    };     
    ($a:ident.y >>= $($tail:tt)*) => {
        concat!(stringify!($a), ".y >>= ", opencl_str!($($tail)*));
    };     
    ($a:ident.y <<= $($tail:tt)*) => {
        concat!(stringify!($a), ".y <<= ", opencl_str!($($tail)*));
    };     

    ($a:ident.z; $($tail:tt)*) => {
        concat!(stringify!($a), ".z;", opencl_str!($($tail)*));
    };
    ($a:ident.z = $($tail:tt)*) => {
        concat!(stringify!($a), ".z = ", opencl_str!($($tail)*));
    };
    ($a:ident.z += $($tail:tt)*) => {
        concat!(stringify!($a), ".z += ", opencl_str!($($tail)*));
    };
    ($a:ident.z -= $($tail:tt)*) => {
        concat!(stringify!($a), ".z -= ", opencl_str!($($tail)*));
    };
    ($a:ident.z *= $($tail:tt)*) => {
        concat!(stringify!($a), ".z *= ", opencl_str!($($tail)*));
    };
    ($a:ident.z /= $($tail:tt)*) => {
        concat!(stringify!($a), ".z /= ", opencl_str!($($tail)*));
    };            
    ($a:ident.z + $($tail:tt)*) => {
        concat!(stringify!($a), ".z + ", opencl_str!($($tail)*));
    };
    ($a:ident.z - $($tail:tt)*) => {
        concat!(stringify!($a), ".z - ", opencl_str!($($tail)*));
    };
    ($a:ident.z * $($tail:tt)*) => {
        concat!(stringify!($a), ".z * ", opencl_str!($($tail)*));
    };
    ($a:ident.z / $($tail:tt)*) => {
        concat!(stringify!($a), ".z / ", opencl_str!($($tail)*));
    };
    ($a:ident.z >> $($tail:tt)*) => {
        concat!(stringify!($a), ".z >> ", opencl_str!($($tail)*));
    };     
    ($a:ident.z << $($tail:tt)*) => {
        concat!(stringify!($a), ".z << ", opencl_str!($($tail)*));
    };     
    ($a:ident.z >>= $($tail:tt)*) => {
        concat!(stringify!($a), ".z >>= ", opencl_str!($($tail)*));
    };     
    ($a:ident.z <<= $($tail:tt)*) => {
        concat!(stringify!($a), ".z <<= ", opencl_str!($($tail)*));
    };         

    ($a:ident.w; $($tail:tt)*) => {
        concat!(stringify!($a), ".w;", opencl_str!($($tail)*));
    };
    ($a:ident.w = $($tail:tt)*) => {
        concat!(stringify!($a), ".w = ", opencl_str!($($tail)*));
    };
    ($a:ident.w += $($tail:tt)*) => {
        concat!(stringify!($a), ".w += ", opencl_str!($($tail)*));
    };
    ($a:ident.w -= $($tail:tt)*) => {
        concat!(stringify!($a), ".w -= ", opencl_str!($($tail)*));
    };
    ($a:ident.w *= $($tail:tt)*) => {
        concat!(stringify!($a), ".w *= ", opencl_str!($($tail)*));
    };
    ($a:ident.w /= $($tail:tt)*) => {
        concat!(stringify!($a), ".w /= ", opencl_str!($($tail)*));
    };            
    ($a:ident.w + $($tail:tt)*) => {
        concat!(stringify!($a), ".w + ", opencl_str!($($tail)*));
    };
    ($a:ident.w - $($tail:tt)*) => {
        concat!(stringify!($a), ".w - ", opencl_str!($($tail)*));
    };
    ($a:ident.w * $($tail:tt)*) => {
        concat!(stringify!($a), ".w * ", opencl_str!($($tail)*));
    };
    ($a:ident.w / $($tail:tt)*) => {
        concat!(stringify!($a), ".w / ", opencl_str!($($tail)*));
    };         
    ($a:ident.w >> $($tail:tt)*) => {
        concat!(stringify!($a), ".w >> ", opencl_str!($($tail)*));
    };     
    ($a:ident.w << $($tail:tt)*) => {
        concat!(stringify!($a), ".w << ", opencl_str!($($tail)*));
    };     
    ($a:ident.w >>= $($tail:tt)*) => {
        concat!(stringify!($a), ".w >>= ", opencl_str!($($tail)*));
    };     
    ($a:ident.w <<= $($tail:tt)*) => {
        concat!(stringify!($a), ".w <<= ", opencl_str!($($tail)*));
    };     

    // image2d_t
    // 
    (image2d_t $a:ident;) => {
        concat!("image2d_t ", stringify!($a), ";");
    };

    // image2d_t
    // 
    (image3d_t $a:ident;) => {
        concat!("image3d_t ", stringify!($a), ";");
    };

    // image2d_array_t
    // 
    (image3d_t $a:ident;) => {
        concat!("image3d_t ", stringify!($a), ";");
    };

    // image1d_t
    // 
    (image1d_t $a:ident;) => {
        concat!("image1d_t ", stringify!($a), ";");
    };

    // image1d_buffer_t
    // 
    (image1d_buffer_t $a:ident;) => {
        concat!("image1d_buffer_t ", stringify!($a), ";");
    };

    // image1d_array_t
    // 
    (image1d_buffer_t $a:ident;) => {
        concat!("image1d_array_t ", stringify!($a), ";");
    };

    // image2d_depth_t
    // 
    (image2d_depth_t $a:ident;) => {
        concat!("image2d_depth_t ", stringify!($a), ";");
    };

    // image2d_array_depth_t
    // 
    (image2d_array_depth_t $a:ident;) => {
        concat!("image2d_array_depth_t ", stringify!($a), ";");
    };

    // sampler_t
    // 
    (sampler_t $a:ident;) => {
        concat!("sampler_t ", stringify!($a), ";");
    };

    // queue_t
    // 
    (queue_t $a:ident;) => {
        concat!("queue_t ", stringify!($a), ";");
    };

    // ndrange_t
    // 
    (ndrange_t $a:ident;) => {
        concat!("ndrange_t ", stringify!($a), ";");
    };

    // clk_event_t
    // 
    (clk_event_t $a:ident;) => {
        concat!("clk_event_t ", stringify!($a), ";");
    };

    // reserve_id_t
    // 
    (reserve_id_t $a:ident;) => {
        concat!("reserve_id_t ", stringify!($a), ";");
    };

    // cl_mem_fence_flags
    // 
    (cl_mem_fence_flags $a:ident;) => {
        concat!("cl_mem_fence_flags ", stringify!($a), ";");
    };

    // event_t
    // 
    (event_t $a:ident;) => {
        concat!("event_t ", stringify!($a), ";");
    };

    (constant $($tail:tt)*) => {
        concat!("constant ", opencl_str!($($tail)*));
    };

    (global $($tail:tt)*) => {
        concat!("global ", opencl_str!($($tail)*));
    };

    (static $($tail:tt)*) => {
        concat!("static ", opencl_str!($($tail)*));
    };

    (local $($tail:tt)*) => {
        concat!("local ", opencl_str!($($tail)*));
    };

    (private $($tail:tt)*) => {
        concat!("private ", opencl_str!($($tail)*));
    };

    (* $($tail:tt)*) => {
        concat!(" * ", opencl_str!($($tail)*));
    };

    (& $($tail:tt)*) => {
        concat!("&", opencl_str!($($tail)*));
    };

    //(for_ ($a:expr; $b:expr; $c:tt) $bk:block) => {
    //    concat!("for(", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")\n", opencl_str!([$bk]));
    //};    
    (for_ ($a:expr; $b:expr; --$c:ident) $bk:block $($tail:tt)* ) => {
        concat!("for(", opencl_str!($a;), opencl_str!($b;), "--", opencl_str!([$c]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ ($a:expr; $b:expr; $c:ident--) $bk:block $($tail:tt)* ) => {
        concat!("for(", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), "--", ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ ($a:expr; $b:expr; ++$c:ident) $bk:block $($tail:tt)* ) => {
        concat!("for(", opencl_str!($a;), opencl_str!($b;), "++", opencl_str!([$c]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ ($a:expr; $b:expr; $c:ident++) $bk:block $($tail:tt)* ) => {
        concat!("for(", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), "++", ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };    
    (for_ ($a:expr; $b:expr; $c:expr) $bk:block) => {
        concat!("for(", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")\n", opencl_str!([$bk]));
    };    
    (for_ ($a:expr; $b:expr; $c:expr) $bk:block $($tail:tt)* ) => {
        concat!("for(", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };

    /*
    (for_ (int $a:expr; $b:expr; $c:tt) $bk:block) => {
        concat!("for( int ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")\n", opencl_str!([$bk]));
    };    
    (for_ (int $a:expr; $b:expr; $c:expr) $bk:block) => {
        concat!("for( int ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")\n", opencl_str!([$bk]));
    };
    */    
    (for_ (int $a:expr; $b:expr; --$c:ident) $bk:block $($tail:tt)* ) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), "--", opencl_str!([$c]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ (int $a:expr; $b:expr; $c:ident--) $bk:block $($tail:tt)* ) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), "--", ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ (int $a:expr; $b:expr; ++$c:ident) $bk:block $($tail:tt)* ) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), "++", opencl_str!([$c]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ (int $a:expr; $b:expr; $c:ident++) $bk:block $($tail:tt)* ) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), "++", ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ (int $a:expr; $b:expr; $c:expr) $bk:block $($tail:tt)* ) => {
        concat!("for( int ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };    

    /*
    (for_ (uint $a:expr; $b:expr; $c:ident++) $bk:block) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")\n", opencl_str!([$bk]));
    };    
    (for_ (uint $a:expr; $b:expr; ++$c:ident) $bk:block) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")\n", opencl_str!([$bk]));
    };        
    (for_ (uint $a:expr; $b:expr; $c:expr) $bk:block) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")\n", opencl_str!([$bk]));
    };    
    */
    (for_ (uint $a:expr; $b:expr; --$c:ident) $bk:block $($tail:tt)* ) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), "--", opencl_str!([$c]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ (uint $a:expr; $b:expr; $c:ident--) $bk:block $($tail:tt)* ) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), "--", ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ (uint $a:expr; $b:expr; ++$c:ident) $bk:block $($tail:tt)* ) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), "++", opencl_str!([$c]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ (uint $a:expr; $b:expr; $c:ident++) $bk:block $($tail:tt)* ) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), "++", ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };
    (for_ (uint $a:expr; $b:expr; $c:expr) $bk:block $($tail:tt)* ) => {
        concat!("for( uint ", opencl_str!($a;), opencl_str!($b;), opencl_str!([$c]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };

    (while_ ($a:expr) $bk:block) => {
        concat!("while(", opencl_str!([$a]), ")\n", opencl_str!([$bk]));
    };    
    (while_ ($a:expr) $bk:block $($tail:tt)* ) => {
        concat!("while(", opencl_str!([$a]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };

    (if_ ($a:expr) $bk:block) => {
        concat!("if(", opencl_str!([$a]), ")\n", opencl_str!([$bk]));
    };    
    (if_ ($a:expr) $bk:block $($tail:tt)* ) => {
        concat!("if(", opencl_str!([$a]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };

    (else_if_ ($a:expr) $bk:block ) => {
        concat!("else if(", opencl_str!([$a]), ")", opencl_str!([$bk]));
    };    
    (else_if_ ($a:expr) $bk:block $($tail:tt)* ) => {
        concat!("else if(", opencl_str!([$a]), ")", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };

    (else_ $bk:block ) => {
        concat!("else", opencl_str!([$bk]));
    };    
    (else_ $bk:block $($tail:tt)* ) => {
        concat!("else", opencl_str!([$bk]), "\n", opencl_str!($($tail)*));
    };

    (union { $($bk:tt)+ } $a:expr;) => {
        concat!("union {\n", opencl_str!($($bk)+), "\n} ", opencl_str!($a;));
    };

    (union { $($bk:tt)+ } $a:expr; $($tail:tt)* ) => {
        concat!("union {\n", opencl_str!($($bk)+), "\n} ", opencl_str!($a;), "\n", opencl_str!($($tail)*));
    };
}

} // end mod
