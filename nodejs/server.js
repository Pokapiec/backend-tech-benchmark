import express from "express";
import pg from "pg";

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
        "Hello": "World",
        "How": "Are You",
        "Today": "My Friend",
        "nested": {
            "key": "value",
            "another_nest": {
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

app.get("/sql/", (req, res) => {
    pool.query('SELECT * FROM public.exampletable ORDER BY id ASC', (error, results) => {
        if (error) {
          throw error
        }
        res.send(results.rows.slice(0, 20));
      })
});


app.listen(PORT, () => 
    console.log("Server running on port: ", PORT)
);