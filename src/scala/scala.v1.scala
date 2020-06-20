import scala.io.Source

object App {
  def main(args: Array[String]): Unit = {
    if (args.length == 0) {
      throw new Exception("Misisng file name")
    }
    val filePath = args(0)
    println(f"processing file:: ${filePath}")

    //
    var count = 0
    for (line <- Source.fromFile(filePath).getLines) {
      count += 1
    }
    println(s"line:: $count")
  }
}