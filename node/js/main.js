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

$(function() {
    const {Renderer} = require('../js/renderer');
    const {Rusty8Client} = require('../js/client');

    // called if connection to rusty8 server has been established
    function onConnected() {
    	console.log('Connected to Rusty8 server.');
    }

    // called when server sedns a render command
    function onRenderCmd() {
    	console.log('Rendering...');
    }

    // connect to server
    const client = new Rusty8Client({
    	host: 			'127.0.0.1',
    	port: 			7890,
    	onConnected: 	onConnected,
    	onRenderCmd: 	onRenderCmd,
    });
    client.connect();
});