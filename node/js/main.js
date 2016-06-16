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

const PIXEL_SIZE = 20;

var keys = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

$(function() {
    const {Renderer} = require('../js/renderer');
    const {Rusty8Client} = require('../js/client');
    const {dialog} = require('electron').remote;
    const fs = require('fs');

    // ==================================================================================
    //                                  Drawing
    // ==================================================================================
    var canvas = document.getElementById("canvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

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
            onRenderCmd:    function(data) {
                //console.log('Rendering... ' + data.length);
                if (canvas.getContext) {
                    var ctx = canvas.getContext("2d");
                    ctx.fillStyle = "rgb(255,255,255)";
                    ctx.clearRect(0, 0, canvas.width, canvas.height);
                    console.log(data.length);
                    for(var i = 0; i < data.length; i++) {
                        var n = data.readUInt8(i);

                        if(n == 0) continue;

                        var x = Math.floor(i % 64);
                        var y = Math.floor(i / 64);
                        ctx.fillStyle = "rgb(255,255,255)";
                        ctx.fillRect (x*PIXEL_SIZE, y*PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
                    }
                }
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

                // update keys
                setInterval(() => {
                    var buf = Buffer.alloc(16);
                    for(var i = 0; i < 16; i++) {
                        buf.writeUInt8(keys[i],i)
                    }
                    client.conn.write(buf);
                }, 1000/17);
            } else {
                connectionErrorModal(true);
            }
        });
    }

    document.addEventListener('keydown', function(event) {
        // 0-9
        for(var i = 0; i < 10; i++) {
            if(event.keyCode == 48 + i) {
                keys[i] = 1;
                return;
            }
        }
        // A-F
        for(var i = 0; i < 6; i++) {
            if(event.keyCode == 65 + i) {
                keys[10+i] = 1;
                return;
            }
        }
    });

    document.addEventListener('keyup', function(event) {
        // 0-9
        for(var i = 0; i < 10; i++) {
            if(event.keyCode == 48 + i) {
                keys[i] = 0;
                return;
            }
        }
        // A-F
        for(var i = 0; i < 6; i++) {
            if(event.keyCode == 65 + i) {
                keys[10+i] = 0;
                return;
            }
        }
    });

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
        });
    });

    // retry conncting to server
    $('#retry-connection').click(() => {
        connect(SERVER_HOST, SERVER_PORT);
    });


});