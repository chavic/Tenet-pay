<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use GuzzleHttp\Client;
use App\Models\User;

class QRcodeGen extends Controller
{
    public function showCode(Request $res)
    {
        $client = new Client();
        $res = $client->request('POST', 'https://url_to_the_api', [
            'json' => [
                'query' => 'generateCode(issure: {{$user->name}}, phoneNumber: {{$user->mobile}}) {
                    code
                  }',
            ]
        ]);

        dd($res);
        echo $res->getStatusCode();
        // 200
        echo $res->getHeader('content-type');
        // 'application/json; charset=utf8'
        echo $res->getBody();
        // {"type":"User"...'
    }
}
