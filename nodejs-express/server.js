import express from "express";
import pg from "pg";
import multer from "multer";
const upload = multer();

// Postgres connection pool
const pool = new pg.Pool({
    user: 'postgres',
    host: 'localhost',
    database: 'postgres',
    password: 'postgres',
    port: 5432,
  })

const app = express();
const PORT = 3000;

app.use(express.json());

app.get("/string/", (req, res) =>{
    res.send("Hello World!");
})

app.get("/simple-json/", (req, res) => {
    const someMoreComplexJson = {
        "key1": "value1",
        "key2": "value2",
        "key3": "value3",
        "key_nest": {
            "kn1": "value_nest_1",
            "knn2": {
                "key": "value"
            }
        }
    };
    res.send(someMoreComplexJson);
});

app.get("/query-params/", (req, res) => {
    const query = req.query;
    res.send(query);
});

app.get("/sql-select/", (req, res) => {
    pool.query('SELECT * FROM public.exampletable ORDER BY id ASC', (error, results) => {
        if (error) {
          throw error
        }
        res.send(results.rows.slice(0, 20));
      })
});

app.post("/file-upload/", upload.any(), (req, res) => {
    const file = req.files[0]
    file.buffer.toString();
    res.send("File upload successful!");
});


app.listen(PORT, () => 
    console.log("Server running on port: ", PORT)
);