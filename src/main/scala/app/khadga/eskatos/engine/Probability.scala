package app.khadga.eskatos.engine

import scala.util.Random

trait Random[T <: Ordering[T]]:
  def roll(): T

trait Die(val size: Int):
  def roll(): Int = Random.between(1, size+1)

