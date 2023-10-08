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
        return "This is a simple text response.";
    }

    @GetMapping("/simple-json/")
    public Map<String, Object> getNestedJson() {
        Map<String, Object> data = new HashMap<>();
        data.put("level1", "value1");

        Map<String, Object> level2 = new HashMap<>();
        level2.put("level2", "value2");

        Map<String, Object> level3 = new HashMap<>();
        level3.put("level3", "value3");

        level2.put("nestedLevel3", level3);
        data.put("nestedLevel2", level2);

        return data;
    }

    @GetMapping("/query-params/")
    public String getQueryParams(@RequestParam("param1") String param1, @RequestParam("param2") String param2) {
        return "Param1: " + param1 + ", Param2: " + param2;
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
        return ResponseEntity.ok("File content:\n" + fileContent);
    }

}
