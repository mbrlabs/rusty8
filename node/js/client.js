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

const net = require('net');

var Rusty8Client = function(options) {
	this.host = options.host;
	this.port = options.port;
	this.onConnected = options.onConnected;
	this.onRenderCmd = options.onRenderCmd;
	this.conn = null;
};

Rusty8Client.prototype.connect = function() {
	console.log('Connecting...');
	this.conn = net.connect(this.port, this.host, () => {
		this.onConnected();

		// receive data
		this.conn.on('data', (data) => {
	        console.log(data.toString());
    	});

		// connection error
	    this.conn.on('error', (ex) => {
	        console.log("handled error");
	        console.log(ex);
	    });

	    // connection end
	    this.conn.on('end', () => {
	        console.log('disconnected from server');
	    });
    });
};

Rusty8Client.prototype.parseData = function(data) {
	// TODO parse
};

Rusty8Client.prototype.sendInput = function(input) {
	// TODO send input data
};

Rusty8Client.prototype.loadRom = function(rom) {
	// TODO load rom
};

module.exports.Rusty8Client = Rusty8Client;