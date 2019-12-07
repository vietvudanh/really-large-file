import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

object App {
  def main(args: Array[String]): Unit = {
    if (args.length == 0) {
      throw new Exception("no filename")
    }
    val fileName = args(0)

    //
    println(f"file::${fileName}")
    val bufferedSource = io.Source.fromFile(fileName)

    var mapDate = mutable.Map[String, Int]().withDefaultValue(0)
    var mapFirstName = mutable.Map[String, Int]()
    var nameIds = new ArrayBuffer[String]()

    var idx = 0
    for(line <- bufferedSource.getLines()) {
      idx += 1

      val Array(_, _, _, _, dateRaw, _, _, name, rest) = line.split("|", 9)
      val date = dateRaw.slice(0, 6)
      mapDate.update(date, mapDate(date) + 1)

      if (idx == 433 || idx == 43244 ) {
        nameIds ++ name
      }

      if (name.contains(", ")) {
        val Array(firstName, lastName, rest) = name.split(", ")
        mapFirstName.update(firstName, mapFirstName(firstName) + 1)
      }
    }

    //
    var maxVal = -1
    var maxFirstName = ""

    for((name, value) <- mapFirstName) {
      if (value > maxVal) {
        maxVal = value
        maxFirstName = name
      }
    }

    println(f"task 1:: ${idx}")
    println("task 2:: " + nameIds.mkString(", "))
    println("task 3 ")
    for((k, v) <- mapDate) {
      println(f"${k}:{v}")
    }
    println("task 4 " + maxFirstName + " " + mapFirstName(maxFirstName))
  }
}