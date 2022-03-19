<?php

use App\Http\Livewire\GenerateComponent;
use Illuminate\Support\Facades\Route;

/*
|--------------------------------------------------------------------------
| Web Routes
|--------------------------------------------------------------------------
|
| Here is where you can register web routes for your application. These
| routes are loaded by the RouteServiceProvider within a group which
| contains the "web" middleware group. Now create something great!
|
*/

Route::get('/', function () {
    return view('welcome');
});

Route::middleware(['auth:sanctum', 'verified'])->group(
    function () {
        Route::get(
            '/dashboard',
            function () {
                return view('dashboard');
            }
        )->name('dashboard');


    Route::get('/generate', GenerateComponent::class)->name('generate-index');

    Route::get('/generate-code', [App\Http\Controllers\QRcodeGen::class, 'showCode'])->name('generate-code');

});
