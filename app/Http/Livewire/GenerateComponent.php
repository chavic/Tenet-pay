<?php

namespace App\Http\Livewire;


use Livewire\Component;

class GenerateComponent extends Component
{
    public function generateCode()
    {
        redirect('/generate-code');
    }

    public function render()
    {
        return view('livewire.generate-component');
    }
}
