
extern crate nscript_lib;
use nscript_lib::*;

/// example struct for injection to nscript interpreter
/// these struct functions can only run on the main thread!!
struct MyCustomStruct {
}
/// A example struct impl for nscript
impl NscriptStructBinding for MyCustomStruct {
    fn nscript_exec(&mut self,tocall:&str,args: &Vec<NscriptVar>, _storage :&mut NscriptStorage) -> NscriptVar{
        let mut retvar = NscriptVar::new("nothing");
        match tocall{
            // in nscript can be called as mystruct::helloworld()
            "helloworld" => {
                print("HelloWorld, from my MyCustomStruct","g");
                // set the return vars string with something
                retvar.stringdata = "var stringdata".to_string();
                // set the return vars string vec with something
                retvar.stringvec.push("vars also are vectors".to_string());
            }
            // print the given argument's stringdata!
            "print" => {
                print(&args[0].stringdata,"b");
            }
            // handle unknown calls if you like
            _ =>{
                print("Ohh cant find the call in mystruct::???","g");
            }
        }
        return  retvar;
    }
}

/// example rust functions see fn main() for injection
/// these also get copied when spawning threads!!
pub fn nscriptfn_helloworld(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("helloworld");
    // set the returning stringdata
    var.stringdata = "helloworld!".to_string();
    // set something to the stringvec
    var.stringvec.push("a var is String and a Vec<String>".to_string());
    // get the data from the argument reference vector
    let _stringdata = storage.getargstring(&args[0], block);
    //get a Stringvector from a argument reference
    let _stringvec = storage.getargstringvec(&args[0], block);
    return var;
}

pub fn main() {
    let mut nscript = Nscript::new();
    // spawn a struct to inject
    let  injectablestruct = MyCustomStruct {};
    // insert a &mut strct into the nscript interpreter
    nscript.insertstructowned("mystruct", injectablestruct);
    // insert a rust function to the interpreter.
    nscript.insertfn("helloworld",nscriptfn_helloworld,"helloworld() prints hello world");
    // run a file given by the terminals 1st argument
    let script = nscript.storage.getglobal("$cmdarg1").stringdata;
    if Nstring::fromright(&script,3) == ".nc" {
        nscript.parsefile(&script);
    }else{
                let mut string = "~/nscript".to_string();
                //var.stringdata
                if let Ok(value) = env::var("NSCRIPT_PATH") {
            //print(&value,"bp");
                    string = value
                }
        //print(&format!("{}/init.nc",string),"r");
        nscript.parsefile(&format!("{}/init.nc",string));

        //print(&format!("{}/init.nc done",string),"r");
    }

    // while theres a coroutine keep the interpreter up!
    // between a cycle of all the coroutine you can add your rust code here.
    loop {
        if nscript.coroutinesindex.len() > 0 {
            // coroutines all run once then the function returns here.
            nscript.executecoroutines();
        }else{
            break;
        }
    }
}
