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
    const fs = require('fs');
    const net = require('net');

    const client = net.connect(7890, '127.0.0.1', () => {
        console.log('connected to server!');
    });

    client.on('data', (data) => {
        console.log(data.toString());
        client.end();
    });

    client.on('error', (ex) => {
        console.log("handled error");
        console.log(ex);
    });

    client.on('end', () => {
        console.log('disconnected from server');
    });

});