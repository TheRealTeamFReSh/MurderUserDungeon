pub const ALL_DIRECTIONS: &str = "
                
            \\                   /
             \\                 /
              \\               / 
              |              / 
              |__            |
              | |\\         __|
              | | \\      /|  |
              | |  \\    / |  |
              | |   \\__|  |  |
              | |   |__|  |  |
              | |   /  |  |  |
              | |  /    \\ |  |
              | | /      \\|  |
              |_|/        |__|
              |              |
              |              | 
              /              \\ 
             /                \\ 
            /                  \\
";

pub const FRONT: &str = "
                
            \\                  /
             \\                /
              \\              / 
               \\            / 
                \\          /   
                |\\        /|
                | \\      / |
                |  \\    /  |
                |   \\__|   |
                |   |__|   |
                |   /  |   |
                |  /    \\  |
                | /      \\ |
                |/        \\|
                /          \\  
               /            \\ 
              /              \\ 
             /                \\ 
            /                  \\
";

pub const LEFT_FRONT: &str = "
                
            \\                  /
             \\                /
              \\              / 
              |             / 
              |__          / 
              | |\\        /
              | | \\      /  
              | |  \\    /   
              | |   \\__|    
              | |   |__|    
              | |   /  |    
              | |  /    \\   
              | | /      \\  
              |_|/        \\
              |            \\
              |             \\ 
              /              \\ 
             /                \\ 
            /                  \\
";

pub const RIGHT_FRONT: &str = "
                
            \\                   /
             \\                 /
              \\               / 
               \\             / 
                \\            |
                 \\         __|
                  \\      /|  |
                   \\    / |  |
                    \\__|  |  |
                    |__|  |  |
                    /  |  |  |
                   /    \\ |  |
                  /      \\|  |
                 /        |__|
                /            |
               /             | 
              /              \\ 
             /                \\ 
            /                  \\
";

pub const LEFT: &str = "
                
            \\                  /
             \\                /
              \\              / 
              |             / 
              |____________/ 
              |            |
              |            | 
              |            | 
              |            | 
              |            | 
              |            | 
              |            | 
              |            | 
              |____________|
              |            \\
              |             \\ 
              /              \\ 
             /                \\ 
            /                  \\
";

pub const RIGHT: &str = "
                
            \\                   /
             \\                 /
              \\               / 
               \\             / 
                \\            |
                 \\___________|
                 |           |
                 |           |
                 |           |
                 |           |
                 |           |
                 |           |
                 |           |
                 |___________|
                /            |
               /             | 
              /              \\ 
             /                \\ 
            /                  \\
";

pub const RAT: &str = "
       _     __,..---''-._                 ';-,
'    _/_),-\"'             '-.                '\\\\\\
\\|.-\"'    -_)                 '.                ||
/'   a   ,                      \\              .'/
'.___,__/                 .-'    \\_        _.-'.'
  |\\  \\      \\         /'        _'\"\"\"\"\"\"'_.-'
     _/;--._, >        |   --.__/ '\"\"\"\"\"\"'
   (((-'  __//''-......-;\\      )
        (((-'       __//  '--. /
                  (((-'    __//
                         (((-'
";

pub const BAT: &str = "
....._      
 '.   ''-.                               .-----.._
   ',     '-.                          .:      /'
     :       '\"..                 ..-''       :
     /   ...--:::'n            n.'::...       :
     ':''      .' ::          /  '.     ''---..:.
       '\\    .'  ._:   .-:   ::    '.     .-''
         :  :    :_\\\\_/: :  .::      '.  /
         : /      \\-../:/_.'-'         \\ :
         :: _.._  q' p ' /'             \\|
         :-'    ''(_. ..-----hh''''''/-._:
                     ':      ''     /     '
                     E:            /
                      :          _/
                      :    _..-''
                      l--''
";

pub const CHEST: &str = "
                             _.--.
                         _.-'_:-'||
                    _.-'_.-::::'||
               _.-:'_.-::::::'  ||
             .''-.-:::::::'     ||
            /.'';|:::::::'      ||_
           ||   ||::::::'     _.;._'-._
           ||   ||:::::'  _.-!oo @.!-._'-.
           \'.  ||:::::.-!()oo @!()@.-'_.|
            '.'-;|:.-'.&$@.& ()$%-'o.'\\U||
              '>'-.!@%()@'@_%-'_.-o _.|'||
               ||-._'-.@.-'_.-' _.-o  |'||
               ||=[ '-._.-\\U/.-'    o |'||
               || '-.]=|| |'|      o  |'||
               ||      || |'|        _| ';
               ||      || |'|    _.-'_.-'
               |'-._   || |'|_.-'_.-'
                '-._'-.|| |' '_.-'
                    '-.||_/.-'
";

pub const KEY: &str = "
        8 8 8 8                     ,ooo.
        8a8 8a8                    oP   ?b
        d888a888zzzzzzzzzzzzzzzzzzzz8     8b
        '\"\" \"\"'                    ?o___oP'
";

pub const SWORD: &str = "
              />
 ()          //---------------------------------------------------------(
(*)OXOXOXOXO(*>                                                          \\
 ()          \\\\-----------------------------------------------------------)
              \\>
";

pub const KNIGHT: &str = "
          / \
          | |
          |.|
          |.|
          |:|      __
        ,_|:|_,   /  )
          (Oo    / _I_
           +\\ \\  || __|
              \\ \\||___|
                \\ /.:.\\-\\
                 |.:. /-----\\
                 |___|::oOo::|
                 /   |:<_T_>:|
                |_____\\ ::: /
                 | |  \\ \\:/
                 | |   | |
                 \\ /   | \\___
                 / |   \\_____\\
                 '-'
";

pub const BOSS: &str = "
              __.......__
            .-:::::::::::::-.
          .:::''':::::::''':::.
        .:::'     ':::'     ':::. 
   .'\\  ::'   ^^^  ':'  ^^^   '::  /'.
  :   \\ ::   _.__       __._   :: /   ;
 :     \\': .' ___\\     /___ '. :'/     ; 
:       /\\   (_|_)\\   /(_|_)   /\\       ;
:      / .\\   __.' ) ( '.__   /. \\      ;
:      \\ (        {   }        ) /      ; 
 :      '-(     .  ^\"^  .     )-'      ;
  '.       \\  .'<'-._.-'>'.  /       .'
    '.      \\    \\;'.';/    /      .'
      '._    '-._       _.-'    _.'
       .''-.__ .''-._.-''. __.-''.
     .'       '.         .'       '.
   .'           '-.   .-'           '.
";
