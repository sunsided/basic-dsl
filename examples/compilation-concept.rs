// Example of how compiled BASIC could look:

use basic_dsl::basic;

basic! {
    10 LET X = 1  
    20 PRINT X
    30 IF X >= 5 THEN GOTO 60
    40 LET X = X + 1
    50 GOTO 20
    60 PRINT "Done"
    70 END
}

// Could compile to something like:
fn compiled_basic() {
    let mut x = 1i64;
    
    'main_loop: loop {
        'label_10: {
            x = 1;
        }
        
        'label_20: {
            println!("{}", x);
        }
        
        'label_30: {
            if x >= 5 {
                break 'label_60;
            }
        }
        
        'label_40: {
            x = x + 1;
        }
        
        'label_50: {
            break 'label_20;
        }
        
        'label_60: {
            println!("Done");
            break 'main_loop;
        }
    }
}