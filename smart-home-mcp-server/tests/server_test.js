// Test suite for Smart Home MCP Server

const http = require('http');
const assert = require('assert');

// Configuration
const SERVER_HOST = 'localhost';
const SERVER_PORT = 3000;
const BASE_URL = `http://${SERVER_HOST}:${SERVER_PORT}`;

// Test utilities
function makeRequest(method, path, data = null) {
    return new Promise((resolve, reject) => {
        const options = {
            hostname: SERVER_HOST,
            port: SERVER_PORT,
            path: path,
            method: method,
            headers: {
                'Content-Type': 'application/json'
            }
        };

        const req = http.request(options, (res) => {
            let responseData = '';
            
            res.on('data', (chunk) => {
                responseData += chunk;
            });
            
            res.on('end', () => {
                try {
                    const parsedData = responseData ? JSON.parse(responseData) : null;
                    resolve({
                        statusCode: res.statusCode,
                        headers: res.headers,
                        data: parsedData,
                        rawData: responseData
                    });
                } catch (e) {
                    resolve({
                        statusCode: res.statusCode,
                        headers: res.headers,
                        data: null,
                        rawData: responseData
                    });
                }
            });
        });

        req.on('error', (err) => {
            reject(err);
        });

        if (data) {
            req.write(JSON.stringify(data));
        }
        
        req.end();
    });
}

// Test cases
async function testHealthEndpoint() {
    console.log('🧪 Testing health endpoint...');
    try {
        const response = await makeRequest('GET', '/health');
        
        assert.strictEqual(response.statusCode, 200, 'Health endpoint should return 200');
        assert.strictEqual(response.data.status, 'OK', 'Health status should be OK');
        assert(response.data.message, 'Health endpoint should return a message');
        
        console.log('✅ Health endpoint test passed');
        return true;
    } catch (error) {
        console.error('❌ Health endpoint test failed:', error.message);
        return false;
    }
}

async function testGpioStatusEndpoint() {
    console.log('🧪 Testing GPIO status endpoint...');
    try {
        const response = await makeRequest('GET', '/api/gpio/status/18');
        
        // We expect this to return an error since we don't have real GPIO hardware
        assert.strictEqual(response.statusCode, 500, 'GPIO status should return 500 without hardware');
        assert(response.data.error, 'GPIO status should return an error message');
        assert(response.data.error.includes('Gpio'), 'Error should mention Gpio');
        
        console.log('✅ GPIO status endpoint test passed');
        return true;
    } catch (error) {
        console.error('❌ GPIO status endpoint test failed:', error.message);
        return false;
    }
}

async function testGpioSetEndpoint() {
    console.log('🧪 Testing GPIO set endpoint...');
    try {
        const response = await makeRequest('POST', '/api/gpio/set/18', { value: 1 });
        
        // We expect this to return an error since we don't have real GPIO hardware
        assert.strictEqual(response.statusCode, 500, 'GPIO set should return 500 without hardware');
        assert(response.data.error, 'GPIO set should return an error message');
        assert(response.data.error.includes('Gpio'), 'Error should mention Gpio');
        
        console.log('✅ GPIO set endpoint test passed');
        return true;
    } catch (error) {
        console.error('❌ GPIO set endpoint test failed:', error.message);
        return false;
    }
}

async function testInvalidEndpoint() {
    console.log('🧪 Testing invalid endpoint...');
    try {
        const response = await makeRequest('GET', '/invalid/endpoint');
        
        assert.strictEqual(response.statusCode, 404, 'Invalid endpoint should return 404');
        
        console.log('✅ Invalid endpoint test passed');
        return true;
    } catch (error) {
        console.error('❌ Invalid endpoint test failed:', error.message);
        return false;
    }
}

async function testGpioWithInvalidPin() {
    console.log('🧪 Testing GPIO with invalid pin...');
    try {
        const response = await makeRequest('GET', '/api/gpio/status/invalid');
        
        // Should still attempt to process but fail
        assert.strictEqual(response.statusCode, 500, 'Invalid pin should return 500');
        
        console.log('✅ Invalid pin test passed');
        return true;
    } catch (error) {
        console.error('❌ Invalid pin test failed:', error.message);
        return false;
    }
}

async function testGpioSetWithoutValue() {
    console.log('🧪 Testing GPIO set without value...');
    try {
        const response = await makeRequest('POST', '/api/gpio/set/18', {});
        
        // Should fail due to missing value
        assert.strictEqual(response.statusCode, 500, 'GPIO set without value should return 500');
        
        console.log('✅ GPIO set without value test passed');
        return true;
    } catch (error) {
        console.error('❌ GPIO set without value test failed:', error.message);
        return false;
    }
}

async function testServerConnectivity() {
    console.log('🧪 Testing server connectivity...');
    try {
        const response = await makeRequest('GET', '/health');
        
        assert(response.statusCode, 'Server should respond with a status code');
        assert(response.headers, 'Server should return headers');
        
        console.log('✅ Server connectivity test passed');
        return true;
    } catch (error) {
        console.error('❌ Server connectivity test failed:', error.message);
        console.error('Make sure the server is running on port 3000');
        return false;
    }
}

// Main test runner
async function runAllTests() {
    console.log('🚀 Starting Smart Home MCP Server Test Suite');
    console.log('=' .repeat(50));
    
    const tests = [
        testServerConnectivity,
        testHealthEndpoint,
        testGpioStatusEndpoint,
        testGpioSetEndpoint,
        testInvalidEndpoint,
        testGpioWithInvalidPin,
        testGpioSetWithoutValue
    ];
    
    let passed = 0;
    let failed = 0;
    
    for (const test of tests) {
        try {
            const result = await test();
            if (result) {
                passed++;
            } else {
                failed++;
            }
        } catch (error) {
            console.error('💥 Test execution error:', error.message);
            failed++;
        }
        console.log(''); // Add spacing between tests
    }
    
    console.log('=' .repeat(50));
    console.log('📊 Test Results Summary:');
    console.log(`✅ Passed: ${passed}`);
    console.log(`❌ Failed: ${failed}`);
    console.log(`📈 Success Rate: ${((passed / (passed + failed)) * 100).toFixed(1)}%`);
    
    if (failed === 0) {
        console.log('🎉 All tests passed! The MCP server is working correctly.');
    } else {
        console.log('⚠️  Some tests failed. Please check the server configuration.');
    }
    
    process.exit(failed > 0 ? 1 : 0);
}

// Handle command line execution
if (require.main === module) {
    runAllTests().catch(error => {
        console.error('💥 Test suite execution failed:', error.message);
        process.exit(1);
    });
}

module.exports = {
    makeRequest,
    testHealthEndpoint,
    testGpioStatusEndpoint,
    testGpioSetEndpoint,
    testInvalidEndpoint,
    testGpioWithInvalidPin,
    testGpioSetWithoutValue,
    testServerConnectivity,
    runAllTests
};