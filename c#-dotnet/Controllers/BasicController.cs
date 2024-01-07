using Microsoft.AspNetCore.Mvc;
using Npgsql;

namespace c__dotnet.Controllers;

[ApiController]
public class BasicController : ControllerBase
{
    private readonly ILogger<BasicController> _logger;
    private readonly IConfiguration _configuration;

    public BasicController(ILogger<BasicController> logger, IConfiguration configuration)
    {
        _logger = logger;
        _configuration = configuration;
    }

    [Route("string")]
    [HttpGet]
    public IActionResult GetString()
    {
        return Ok("Hello world!");
    }

    [Route("simple-json")]
    [HttpGet]
    public IActionResult GetNestedJson()
    {
        var nestedJson = new Dictionary<string, object>
        {
            {"key1", "value1"},
            {"key2", "value2"},
            {"key3", "value3"},
            {"key_nest", new Dictionary<string, object>
                {
                    {"kn1", "value_nest_1"},
                    {"knn2", new Dictionary<string, object>
                        {
                            {"key", "value"}
                        }
                    }
                }
            }
        };

        return Ok(nestedJson);
    }

    [Route("query-params")]
    [HttpGet]
    public IActionResult GetQueryParams([FromQuery] Dictionary<string, string> queryParams)
    {
        return Ok(queryParams);
    }

    [Route("sql-select")]
    [HttpGet]
    public IActionResult GetExampleData()
    {
        var connectionString = _configuration.GetConnectionString("DefaultConnection");

        using (var connection = new NpgsqlConnection(connectionString))
        {
            connection.Open();

            using (var command = new NpgsqlCommand("SELECT * FROM public.exampletable ORDER BY id ASC", connection))
            {
                using (var reader = command.ExecuteReader())
                {
                    // Process the query results here
                    // Example: Read data and return as JSON
                    var resultList = new List<object>();
                    while (reader.Read())
                    {
                        var row = new
                        {
                            ID = reader["ID"],
                            FirstName = reader["FirstName"],
                            LastName = reader["LastName"],
                            Age = reader["Age"],
                            Salary = reader["Salary"],
                            BirthDate = reader["BirthDate"],
                            IsActive = reader["IsActive"],
                            Email = reader["Email"],
                            PhoneNumber = reader["PhoneNumber"],
                            Address = reader["Address"],
                        };
                        resultList.Add(row);
                    }
                    return Ok(resultList);
                }
            }
        }
    }

    [Route("file-upload")]
    [HttpPost]
    public async Task<IActionResult> UploadFile(IFormFile file)
    {
        if (file != null && file.Length > 0)
        {
            using (var streamReader = new StreamReader(file.OpenReadStream()))
            {
                var fileContent = await streamReader.ReadToEndAsync();
                return Ok(fileContent);
            }
        }
        else
        {
            return BadRequest("No file uploaded.");
        }
    }

}
