package com.masters.perftest.api;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.multipart.MultipartFile;

@RestController
public class SimpleApiRestController {

    @Autowired
    private JdbcTemplate jdbcTemplate;

    @GetMapping("/string/")
    public String getText() {
        return "Hello world!";
    }

    @GetMapping("/simple-json/")
    public Map<String, Object> getNestedJson() {
        Map<String, Object> data = new HashMap<>();
        data.put("key1", "value1");
        data.put("key2", "value2");
        data.put("key3", "value3");

        Map<String, Object> nestedData = new HashMap<>();
        nestedData.put("kn1", "value_nest_1");

        Map<String, Object> nestedNestedData = new HashMap<>();
        nestedNestedData.put("key", "value");

        nestedData.put("knn2", nestedNestedData);

        data.put("key_nest", nestedData);

        return data;
    }

    @GetMapping("/query-params/")
    public Map<String, String> getQueryParams(@RequestParam("param1") String param1,
            @RequestParam("param2") String param2) {

        Map<String, String> data = new HashMap<>();
        data.put("param1", param1);
        data.put("param2", param2);
        return data;
    }

    @GetMapping("/sql-select/")
    public List<Map<String, Object>> executeSqlQuery() {
        String sqlQuery = "SELECT * FROM public.exampletable ORDER BY id ASC";
        return jdbcTemplate.queryForList(sqlQuery);
    }

    @PostMapping("/file-upload/")
    public ResponseEntity<String> uploadFile(@RequestParam("file") MultipartFile file) throws IOException {
        byte[] fileBytes = file.getBytes();
        String fileContent = new String(fileBytes);
        return ResponseEntity.ok(fileContent);
    }

}
