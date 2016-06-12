// This file is part of rusty8.
// 
// rusty8 is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.
// 
// rusty8 is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with rusty8. If not, see <http://www.gnu.org/licenses/>.

const SERVER_HOST = '127.0.0.1';
const SERVER_PORT = 7890;

$(function() {
    const {Renderer} = require('../js/renderer');
    const {Rusty8Client} = require('../js/client');
    const {dialog} = require('electron').remote;

    var client = null;
    function connect(host, port) {
        // build client
        client = new Rusty8Client({
            host:           SERVER_HOST,
            port:           SERVER_PORT,
            onRenderCmd:    function() {
                console.log('Rendering...');
            },
            onEnd:          function() {
                $('#no-conn-modal').openModal();
            },
        });

        // connect to backend
        client.connect((success) => {
            console.log('Connected to server');
        });
    }

    // load rom
    $('#load-rom').click({properties: ['openFile']}, () => {
        dialog.showOpenDialog({properties: ['openFile']}, (file) => {
            alert(file);
        });
    });


});