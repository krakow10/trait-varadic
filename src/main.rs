trait Arg1{
	fn varadic(&self,a:&str);
}
trait Arg2{
	fn varadic(&self,a:&str,b:&str);
}

struct Varadic;
impl Arg1 for Varadic{
	fn varadic(&self,a:&str) {
		println!("Called with one argument: {a}");
	}
}
impl Arg2 for Varadic{
	fn varadic(&self,a:&str,b:&str) {
		println!("Called with two arguments: {a}, {b}");
	}
}

fn main() {
	Varadic.varadic("yo");
	Varadic.varadic("yo","yo");
}
