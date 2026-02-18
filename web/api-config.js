// Auto-generated API configuration
// This file is dynamically loaded by the frontend
(function() {
    'use strict';
    
    // Clear any old ngrok URLs from localStorage
    const currentStored = localStorage.getItem('api_base');
    if (currentStored && currentStored.includes('ngrok')) {
        console.log('Removing old ngrok URL from localStorage:', currentStored);
        localStorage.removeItem('api_base');
        localStorage.removeItem('api_base_user_configured');
    }
    
    // Determine the correct API base URL
    let apiBase;
    
    if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
        // Local development
        apiBase = 'http://localhost:8000';
    } else {
        // Production or tunnel - use relative URL or configured endpoint
        // For Cloudflare/ngrok tunnels, we need to use the same origin or configured proxy
        apiBase = localStorage.getItem('api_base') || '';
    }
    
    // If we're on HTTPS but trying to access HTTP localhost, warn about mixed content
    if (window.location.protocol === 'https:' && apiBase.startsWith('http:')) {
        console.warn('Mixed content warning: HTTPS page trying to access HTTP API. This will be blocked by browsers.');
        console.warn('For tunnel access, configure API_BASE to use HTTPS or a relative URL.');
    }
    
    window.API_CONFIG = {
        API_BASE: apiBase,
        PUBLIC_URL: apiBase,
        VERSION: '3.0.0',
        TIMESTAMP: new Date().toISOString()
    };
    
    // Set localStorage for consistency (only for localhost)
    if (apiBase && (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1')) {
        localStorage.setItem('api_base', apiBase);
    }
    
    console.log('API_CONFIG loaded:', window.API_CONFIG);
})();
