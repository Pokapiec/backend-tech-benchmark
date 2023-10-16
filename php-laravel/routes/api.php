<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;

/*
|--------------------------------------------------------------------------
| API Routes
|--------------------------------------------------------------------------
|
| Here is where you can register API routes for your application. These
| routes are loaded by the RouteServiceProvider and all of them will
| be assigned to the "api" middleware group. Make something great!
|
*/

Route::middleware('auth:sanctum')->get('/user', function (Request $request) {
    return $request->user();
});
Route::get('/string', 'App\Http\Controllers\ApiController@getString');
Route::get('/simple-json', 'App\Http\Controllers\ApiController@getSimpleJson');
Route::get('/query-params', 'App\Http\Controllers\ApiController@getQueryParams');
Route::get('/sql-select', 'App\Http\Controllers\ApiController@getSqlSelect');
Route::post('/file-upload', 'App\Http\Controllers\ApiController@postFileUpload');