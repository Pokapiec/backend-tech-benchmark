express = require("express");
pg = require("pg");
multer = require("multer");
const cluster = require('cluster');
const numCPUs = require('os').cpus().length;
const upload = multer();
const PORT = 80;

// Postgres connection pool
const pool = new pg.Pool({
    user: 'postgres',
    host: '10.101.179.3',
    database: 'postgres',
    password: 'postgres',
    port: 5432,
});

if (cluster.isMaster) {
    console.log(`Master ${process.pid} is running`);

    // Fork workers.
    for (let i = 0; i < numCPUs; i++) {
        cluster.fork();
    }

    cluster.on('exit', (worker, code, signal) => {
        console.log(`worker ${worker.process.pid} died`);
    });
} else {
    const app = express();

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
        res.send(file.buffer.toString());
    });


    app.listen(PORT, () => 
        console.log("Server running on port: ", PORT)
    );
}

