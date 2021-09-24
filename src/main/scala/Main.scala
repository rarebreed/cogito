import scala.util.Try


@main def hello: Unit = 
  println("Hello world!")
  println(msg)
  println(test2)

def msg = "I was compiled by Scala 3. :)"


// Using an enum for a linked list
enum MList[+A]:
  case LNil
  case Cons(head: A, tail: MList[A] = LNil)

import MList.{LNil, Cons}

def test(): MList[Int] = Cons(1, Cons(2, Cons(3, LNil)))


trait Iterable[F[_]]:
  extension[F[_], A](x: F[A])
    def next(): Option[A]

// This is sort of like compose, except left to right execution    
extension[A](a : A)
  def |>[B](f: A => B): B = f(a) 
 
def test2 = 
  val foo = { (x: Int) => 
      x * 2
  }

  val bar = { (y: Int) =>
    y + 10
  }

  foo(3) |> bar |> {x => x + 7}

trait Functor[F[_]]:
  def map[A, B](a: F[A], f: A => B): F[B]