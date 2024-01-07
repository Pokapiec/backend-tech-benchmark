<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;

class ApiController extends Controller
{
    public function getString()
    {
        return 'Hello World!';
    }

    public function getSimpleJson()
    {
        $data = [
            'key1' => 'value1',
            'key2' => 'value2',
            'key3' => 'value3',
            'key_nest' => [
                'kn1' => 'value_nest_1',
                'knn2' => [
                    'key' => 'value',
                ],
            ],
        ];

        return response()->json($data);
    }

    public function getQueryParams(Request $request)
    {
        $queryParams = $request->all();
        return $queryParams;
    }

    public function getSqlSelect()
    {
        $result = DB::select('SELECT * FROM public.exampletable ORDER BY id ASC');
        return $result;
    }

    public function postFileUpload(Request $request)
    {
        if ($request->hasFile('file')) {
            $file = $request->file('file');
            $contents = file_get_contents($file);
            return $contents;
        }

        return 'No file uploaded.';
    }
}