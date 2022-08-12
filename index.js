// A very ugly way to serve a rustdoc

import express from "express"
import glob from "fast-glob"

const app = express()

for (const path of [
    ...glob.sync(
        "target/doc/**/*",
        { onlyDirectories: true }
    ),
    "target/doc"
]) {
  console.log(path)
  app.use(express.static(path))
}

app.listen(8080)