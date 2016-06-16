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

var connectionErrorModalOpen = false;

$(function() {
    const {Renderer} = require('../js/renderer');
    const {Rusty8Client} = require('../js/client');
    const {dialog} = require('electron').remote;
    const fs = require('fs');

    // ==================================================================================
    //                                  Modals
    // ==================================================================================

    function connectionErrorModal(open) {
        if(!connectionErrorModalOpen && open) {
            $('#no-conn-modal').openModal({
                dismissible: false,
            });
            connectionErrorModalOpen = true;
        } else if(!open) {
            $('#no-conn-modal').closeModal();
            connectionErrorModalOpen = false;
        }
    }

    // ==================================================================================
    //                                  Networking
    // ==================================================================================

    var client = null;
    function connect(host, port, romBuffer) {
        // build client
        client = new Rusty8Client({
            host:           SERVER_HOST,
            port:           SERVER_PORT,
            onRenderCmd:    function() {
                console.log('Rendering...');
            },
            onEnd:          function() {
                //connectionErrorModal(true);
            },
            onError:        function() {
                //connectionErrorModal(true);
            },
        });

        // connect to backend
        client.connect((success) => {
            if(success) {
                console.log('Connected to server');
                connectionErrorModal(false);

                var len = romBuffer.length;
                var buf = new Buffer.alloc(2);
                buf.writeUInt16BE(len, 0);

                // send rom
                client.conn.write(buf);
                client.conn.write(romBuffer);
            } else {
                connectionErrorModal(true);
            }
        });
    }

    // ==================================================================================
    //                              UI stuff
    // ==================================================================================
        

    // load rom
    $('#load-rom').click(() => {
        dialog.showOpenDialog({properties: ['openFile']}, (file) => {
            var romData = fs.readFileSync(file[0]);
            var len = romData.length;
            console.log("Loaded rom " + file[0] + " => " + len + " bytes");
            connect(SERVER_HOST, SERVER_PORT, romData);
            alert(file);
        });
    });

    // retry conncting to server
    $('#retry-connection').click(() => {
        connect(SERVER_HOST, SERVER_PORT);
    });

    // ==================================================================================
    //                                  Main
    // ==================================================================================
    //connect(SERVER_HOST, SERVER_PORT);


});