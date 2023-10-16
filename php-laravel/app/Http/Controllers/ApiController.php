<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;

class ApiController extends Controller
{
    public function getString()
    {
        return 'Simple Text Content';
    }

    public function getSimpleJson()
    {
        $data = [
            'level1' => [
                'level2' => [
                    'level3' => 'Nested JSON Example',
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