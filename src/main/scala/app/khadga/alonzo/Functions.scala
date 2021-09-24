package app.khadga.alonzo

def compose[A, B, C](fn1: A => B)(fn2: B => C): A => C = { (a: A) =>
  fn2(fn1(a))
}

extension[A, B](fn1: A => B)
  infix def |>[C](fn2: B => C): A => C = compose(fn1)(fn2)

object Test:
  def test() = {
    val f: Int => Int = _ * 2
    val g: Int => String = x => x.toString

    val fn = f |> g
    val answer = fn(10)
  }

// applies the first argument to a function taking 2 args.  The type of the first
// arg in the function must match the type of the presupplied arg
def partial[A, B, C](a: A, f: (A, B) => C): B => C = 
  (b: B) => f(a, b)

// returns a curried version of a function that takes 2 args
def curry[A, B, C](f: (A, B) => C): (a: A) => (b: B) => C = 
  a => b => f(a, b)

def curry3[A, B, C, D](f: (A, B, C) => D): A => B => C => D = 
  a => b => c => f(a, b, c)

def uncurry[A, B, C](f: A => B => C): (A, B) => C = 
  (a: A, b: B) => f(a)(b)


@main def main = 
  def foo(x: Int, y: Int): Int = x * y
  val b = curry(foo)
  val ans = b(2)(4)
  println(ans)

  val f = uncurry(b)
  println(f(3, 4))