#!/usr/bin/env python3
"""
MCP Server for Windows Security Monitoring
Mục tiêu: Bảo vệ máy tính khỏi bị tấn công từ mạng internet
"""

import asyncio
import json
import logging
import os
import sys
from typing import Any, Dict, List, Optional
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

# MCP server imports
from mcp.server import Server
from mcp.server.models import InitializationOptions
from mcp.server.stdio import stdio_server
from mcp.server.lowlevel.server import NotificationOptions
from mcp.types import (
    Resource,
    Tool,
    TextContent,
    ImageContent,
    EmbeddedResource,
    LoggingLevel
)

# Windows specific imports
import platform
import psutil
import subprocess
import time
from datetime import datetime
import threading
from pathlib import Path

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("windows-security-server")

class WindowsSecurityServer:
    """MCP Server for Windows Security Monitoring with Advanced Features"""
    
    def __init__(self):
        # Load configuration from environment variables
        self.server_name = os.getenv("SERVER_NAME", "windows-security-server")
        self.cpu_threshold = float(os.getenv("CPU_THRESHOLD", "70"))
        self.memory_threshold = float(os.getenv("MEMORY_THRESHOLD", "70"))
        self.monitor_interval = int(os.getenv("MONITOR_INTERVAL", "60"))
        self.firewall_timeout = int(os.getenv("FIREWALL_AUTO_CLOSE_TIMEOUT", "300"))
        
        # Security features flags
        self.enable_firewall_monitoring = os.getenv("ENABLE_FIREWALL_MONITORING", "true").lower() == "true"
        self.enable_process_monitoring = os.getenv("ENABLE_PROCESS_MONITORING", "true").lower() == "true"
        self.enable_network_monitoring = os.getenv("ENABLE_NETWORK_MONITORING", "true").lower() == "true"
        self.log_security_events = os.getenv("LOG_SECURITY_EVENTS", "true").lower() == "true"
        
        # Advanced security features
        self.active_firewall_rules = {}  # Track temporary firewall rules
        self.threat_patterns = self._load_threat_patterns()
        self.security_alerts = []
        self.auto_response_enabled = os.getenv('AUTO_RESPONSE_ENABLED', 'true').lower() == 'true'
        self.rule_timeout_seconds = int(os.getenv('FIREWALL_RULE_TIMEOUT', '3600'))  # 1 hour default
        
        self.server = Server(self.server_name)
        self.setup_handlers()
        
        # Start background monitoring
        self._start_background_monitoring()
        
    def setup_handlers(self):
        """Setup MCP server handlers"""
        
        @self.server.list_resources()
        async def handle_list_resources() -> List[Resource]:
            """List available resources"""
            return [
                Resource(
                    uri="windows://system-info",
                    name="Windows System Information",
                    description="Current Windows system information and security status",
                    mimeType="application/json",
                ),
                Resource(
                    uri="windows://processes",
                    name="Running Processes",
                    description="List of currently running processes with resource usage",
                    mimeType="application/json",
                ),
                Resource(
                    uri="windows://firewall-status",
                    name="Firewall Status",
                    description="Current Windows Firewall configuration and status",
                    mimeType="application/json",
                ),
            ]

        @self.server.read_resource()
        async def handle_read_resource(uri: str) -> str:
            """Read a specific resource"""
            if uri == "windows://system-info":
                return await self.get_system_info()
            elif uri == "windows://processes":
                return await self.get_processes_info()
            elif uri == "windows://firewall-status":
                return await self.get_firewall_status()
            else:
                raise ValueError(f"Unknown resource: {uri}")

        @self.server.list_tools()
        async def handle_list_tools() -> List[Tool]:
            """List available tools"""
            return [
                Tool(
                    name="monitor_system_resources",
                    description="Monitor CPU and RAM usage, alert if usage exceeds 70%",
                    inputSchema={
                        "type": "object",
                        "properties": {
                            "threshold": {
                                "type": "number",
                                "description": "CPU/RAM usage threshold (default: 70%)",
                                "default": 70
                            }
                        }
                    },
                ),
                Tool(
                    name="get_windows_security_status",
                    description="Get comprehensive Windows security status",
                    inputSchema={
                        "type": "object",
                        "properties": {}
                    },
                ),
                Tool(
                    name="check_firewall_rules",
                    description="Check current firewall rules and configuration",
                    inputSchema={
                        "type": "object",
                        "properties": {}
                    },
                ),
            ]

        @self.server.call_tool()
        async def handle_call_tool(name: str, arguments: Dict[str, Any]) -> List[TextContent]:
            """Handle tool calls"""
            if name == "monitor_system_resources":
                threshold = arguments.get("threshold", 70)
                result = await self.monitor_system_resources(threshold)
                return [TextContent(type="text", text=json.dumps(result, indent=2, ensure_ascii=False))]
            
            elif name == "get_windows_security_status":
                result = await self.get_windows_security_status()
                return [TextContent(type="text", text=json.dumps(result, indent=2, ensure_ascii=False))]
            
            elif name == "check_firewall_rules":
                result = await self.check_firewall_rules()
                
            # Advanced security tools
            elif name == "create_firewall_rule":
                return await self.create_firewall_rule_tool(arguments)
            elif name == "get_security_alerts":
                return await self.get_security_alerts_tool(arguments)
            elif name == "manage_auto_response":
                return await self.manage_auto_response_tool(arguments)
            elif name == "scan_network_connections":
                return await self.scan_network_connections_tool(arguments)
            elif name == "get_active_firewall_rules":
                return await self.get_active_firewall_rules_tool(arguments)
            
            else:
                raise ValueError(f"Unknown tool: {name}")

    async def get_system_info(self) -> str:
        """Get Windows system information"""
        try:
            info = {
                "system": platform.system(),
                "release": platform.release(),
                "version": platform.version(),
                "machine": platform.machine(),
                "processor": platform.processor(),
                "hostname": platform.node(),
                "timestamp": datetime.now().isoformat(),
                "cpu_count": psutil.cpu_count(),
                "memory_total_gb": round(psutil.virtual_memory().total / (1024**3), 2),
                "disk_usage": {
                    "total_gb": round(psutil.disk_usage('C:').total / (1024**3), 2),
                    "used_gb": round(psutil.disk_usage('C:').used / (1024**3), 2),
                    "free_gb": round(psutil.disk_usage('C:').free / (1024**3), 2),
                }
            }
            return json.dumps(info, indent=2, ensure_ascii=False)
        except Exception as e:
            logger.error(f"Error getting system info: {e}")
            return json.dumps({"error": str(e)}, ensure_ascii=False)

    async def get_processes_info(self) -> str:
        """Get information about running processes"""
        try:
            processes = []
            for proc in psutil.process_iter(['pid', 'name', 'cpu_percent', 'memory_percent']):
                try:
                    processes.append(proc.info)
                except (psutil.NoSuchProcess, psutil.AccessDenied):
                    pass
            
            # Sort by CPU usage
            processes.sort(key=lambda x: x['cpu_percent'] or 0, reverse=True)
            
            result = {
                "timestamp": datetime.now().isoformat(),
                "total_processes": len(processes),
                "top_10_cpu": processes[:10],
                "high_resource_processes": [
                    p for p in processes 
                    if (p['cpu_percent'] or 0) > 70 or (p['memory_percent'] or 0) > 70
                ]            }
            return json.dumps(result, indent=2, ensure_ascii=False)
        except Exception as e:
            logger.error(f"Error getting processes info: {e}")
            return json.dumps({"error": str(e)}, ensure_ascii=False)

    async def get_firewall_status(self) -> str:
        """Get Windows Firewall status"""
        try:
            # Run Windows command to get firewall status
            result = subprocess.run(
                ["netsh", "advfirewall", "show", "allprofiles"],
                capture_output=True,
                text=True,
                shell=True
            )
            
            firewall_info = {
                "timestamp": datetime.now().isoformat(),
                "command_output": result.stdout,
                "error": result.stderr if result.stderr else None,
                "return_code": result.returncode
            }
            return json.dumps(firewall_info, indent=2, ensure_ascii=False)
        except Exception as e:
            logger.error(f"Error getting firewall status: {e}")
            return json.dumps({"error": str(e)}, ensure_ascii=False)

    async def monitor_system_resources(self, threshold: float = None) -> Dict[str, Any]:
        """Monitor system resources and alert if usage exceeds threshold"""
        if threshold is None:
            threshold = self.cpu_threshold
            
        try:
            cpu_percent = psutil.cpu_percent(interval=1)
            memory = psutil.virtual_memory()
            memory_percent = memory.percent
            
            result = {
                "timestamp": datetime.now().isoformat(),
                "threshold": threshold,
                "cpu_usage": cpu_percent,
                "memory_usage": memory_percent,
                "alerts": []
            }
            
            if cpu_percent > threshold:
                result["alerts"].append(f"HIGH CPU USAGE: {cpu_percent:.1f}% (threshold: {threshold}%)")
            
            if memory_percent > threshold:
                result["alerts"].append(f"HIGH MEMORY USAGE: {memory_percent:.1f}% (threshold: {threshold}%)")
            
            # Get top processes consuming resources
            high_usage_processes = []
            for proc in psutil.process_iter(['pid', 'name', 'cpu_percent', 'memory_percent']):
                try:
                    if (proc.info['cpu_percent'] or 0) > threshold or (proc.info['memory_percent'] or 0) > threshold:
                        high_usage_processes.append(proc.info)
                except (psutil.NoSuchProcess, psutil.AccessDenied):
                    pass
            
            result["high_usage_processes"] = high_usage_processes
            
            return result
        except Exception as e:
            logger.error(f"Error monitoring system resources: {e}")
            return {"error": str(e)}

    async def get_windows_security_status(self) -> Dict[str, Any]:
        """Get comprehensive Windows security status"""
        try:
            result = {
                "timestamp": datetime.now().isoformat(),
                "system_info": json.loads(await self.get_system_info()),
                "firewall_status": "Checking...",
                "windows_defender": "Checking...",
                "network_connections": []
            }
            
            # Get network connections
            connections = psutil.net_connections(kind='inet')
            result["network_connections"] = [
                {
                    "local_address": f"{conn.laddr.ip}:{conn.laddr.port}" if conn.laddr else "N/A",
                    "remote_address": f"{conn.raddr.ip}:{conn.raddr.port}" if conn.raddr else "N/A",
                    "status": conn.status,
                    "pid": conn.pid
                }
                for conn in connections[:20]  # Limit to first 20 connections
            ]
            
            return result
        except Exception as e:
            logger.error(f"Error getting security status: {e}")
            return {"error": str(e)}

    async def check_firewall_rules(self) -> Dict[str, Any]:
        """Check firewall rules"""
        try:
            # Get basic firewall status
            result = subprocess.run(
                ["netsh", "advfirewall", "show", "currentprofile"],
                capture_output=True,
                text=True,
                shell=True            )
            
            return {
                "timestamp": datetime.now().isoformat(),
                "firewall_profile": result.stdout,
                "status": "active" if result.returncode == 0 else "error",
                "error": result.stderr if result.stderr else None
            }
        except Exception as e:
            logger.error(f"Error checking firewall rules: {e}")
            return {"error": str(e)}
    
    # Advanced Security Tool Methods
    async def create_firewall_rule_tool(self, arguments: Dict[str, Any]) -> Dict[str, Any]:
        """Create a temporary firewall rule with auto-expiration"""
        try:
            program_path = arguments.get("program_path")
            action = arguments.get("action", "block")
            duration_minutes = arguments.get("duration_minutes", 60)
            custom_name = arguments.get("rule_name")
            
            # Generate rule name
            timestamp = int(time.time())
            rule_name = custom_name or f"MCP_TEMP_{action.upper()}_{timestamp}"
            
            # Create the firewall rule
            success = self._create_firewall_rule(rule_name, program_path, action)
            
            if success:
                # Schedule rule removal
                expires_at = time.time() + (duration_minutes * 60)
                self.active_firewall_rules[rule_name] = {
                    'created_at': time.time(),
                    'expires_at': expires_at,
                    'program_path': program_path,
                    'action': action,
                    'duration_minutes': duration_minutes
                }
                
                return {
                    "success": True,
                    "rule_name": rule_name,
                    "program_path": program_path,
                    "action": action,
                    "expires_at": datetime.fromtimestamp(expires_at).isoformat(),
                    "message": f"Firewall rule '{rule_name}' created successfully"
                }
            else:
                return {
                    "success": False,
                    "error": "Failed to create firewall rule"
                }
                
        except Exception as e:
            logger.error(f"Error creating firewall rule: {e}")
            return {"error": str(e)}
    
    async def get_security_alerts_tool(self, arguments: Dict[str, Any]) -> Dict[str, Any]:
        """Get recent security alerts"""
        try:
            limit = arguments.get("limit", 50)
            alert_type = arguments.get("alert_type")
            
            alerts = self.security_alerts[-limit:] if not alert_type else [
                alert for alert in self.security_alerts[-limit*2:]
                if alert.get('type') == alert_type
            ][-limit:]
            
            return {
                "timestamp": datetime.now().isoformat(),
                "total_alerts": len(self.security_alerts),
                "returned_alerts": len(alerts),
                "alerts": alerts,
                "alert_types": list(set(alert.get('type') for alert in self.security_alerts))
            }
            
        except Exception as e:
            logger.error(f"Error getting security alerts: {e}")
            return {"error": str(e)}
    
    async def manage_auto_response_tool(self, arguments: Dict[str, Any]) -> Dict[str, Any]:
        """Manage automated security response settings"""
        try:
            enabled = arguments.get("enabled")
            timeout_minutes = arguments.get("timeout_minutes")
            
            if enabled is not None:
                self.auto_response_enabled = enabled
                
            if timeout_minutes is not None:
                self.rule_timeout_seconds = timeout_minutes * 60
            
            return {
                "timestamp": datetime.now().isoformat(),
                "auto_response_enabled": self.auto_response_enabled,
                "rule_timeout_minutes": self.rule_timeout_seconds // 60,
                "message": "Auto-response settings updated successfully"
            }
            
        except Exception as e:
            logger.error(f"Error managing auto-response: {e}")
            return {"error": str(e)}
    
    async def scan_network_connections_tool(self, arguments: Dict[str, Any]) -> Dict[str, Any]:
        """Scan for suspicious network connections"""
        try:
            include_established = arguments.get("include_established", True)
            include_listening = arguments.get("include_listening", True)
            
            suspicious_connections = []
            all_connections = []
            
            for conn in psutil.net_connections():
                if conn.status == 'ESTABLISHED' and not include_established:
                    continue
                if conn.status == 'LISTEN' and not include_listening:
                    continue
                
                conn_info = {
                    'local_address': f"{conn.laddr.ip}:{conn.laddr.port}" if conn.laddr else "N/A",
                    'remote_address': f"{conn.raddr.ip}:{conn.raddr.port}" if conn.raddr else "N/A",
                    'status': conn.status,
                    'pid': conn.pid,
                    'process_name': None
                }
                
                # Get process name
                try:
                    if conn.pid:
                        proc = psutil.Process(conn.pid)
                        conn_info['process_name'] = proc.name()
                except (psutil.NoSuchProcess, psutil.AccessDenied):
                    pass
                
                all_connections.append(conn_info)
                
                # Check for suspicious ports
                if conn.laddr and conn.laddr.port in self.threat_patterns['suspicious_network_ports']:
                    conn_info['suspicious_reason'] = f"Suspicious port {conn.laddr.port}"
                    suspicious_connections.append(conn_info)
                
                # Check for external connections from suspicious processes
                if (conn_info['process_name'] and 
                    any(pattern in conn_info['process_name'].lower() 
                        for pattern in self.threat_patterns['suspicious_connections'])):
                    conn_info['suspicious_reason'] = f"Suspicious process: {conn_info['process_name']}"
                    suspicious_connections.append(conn_info)
            
            return {
                "timestamp": datetime.now().isoformat(),
                "total_connections": len(all_connections),
                "suspicious_connections": len(suspicious_connections),
                "connections": all_connections,
                "suspicious_details": suspicious_connections
            }
            
        except Exception as e:
            logger.error(f"Error scanning network connections: {e}")
            return {"error": str(e)}
    
    async def get_active_firewall_rules_tool(self, arguments: Dict[str, Any]) -> Dict[str, Any]:
        """Get currently active temporary firewall rules"""
        try:
            current_time = time.time()
            active_rules = []
            
            for rule_name, rule_info in self.active_firewall_rules.items():
                remaining_seconds = rule_info['expires_at'] - current_time
                remaining_minutes = max(0, remaining_seconds / 60)
                
                active_rules.append({
                    'rule_name': rule_name,
                    'program_path': rule_info['program_path'],
                    'action': rule_info.get('action', 'block'),
                    'created_at': datetime.fromtimestamp(rule_info['created_at']).isoformat(),
                    'expires_at': datetime.fromtimestamp(rule_info['expires_at']).isoformat(),
                    'remaining_minutes': round(remaining_minutes, 1),
                    'threat_info': rule_info.get('threat_info')
                })
            
            return {
                "timestamp": datetime.now().isoformat(),
                "active_rules_count": len(active_rules),
                "active_rules": active_rules
            }
            
        except Exception as e:
            logger.error(f"Error getting active firewall rules: {e}")
            return {"error": str(e)}

    # ...existing code...
    
    def _load_threat_patterns(self) -> Dict[str, Any]:
        """Load threat detection patterns"""
        return {
            'suspicious_processes': [
                'powershell.exe -enc',
                'cmd.exe /c whoami',
                'netstat -an',
                'tasklist /svc',
                'systeminfo',
                'wmic'
            ],
            'suspicious_connections': [
                'nc.exe',
                'netcat',
                'psexec',
                'mimikatz'
            ],
            'high_cpu_threshold': 90,
            'high_memory_threshold': 85,
            'suspicious_network_ports': [4444, 5555, 6666, 8080, 9999]
        }
    
    def _start_background_monitoring(self):
        """Start background security monitoring"""
        def monitor_loop():
            while True:
                try:
                    self._monitor_threats()
                    self._cleanup_expired_firewall_rules()
                    time.sleep(30)  # Check every 30 seconds
                except Exception as e:
                    logger.error(f"Background monitoring error: {e}")
                    time.sleep(60)  # Wait longer on error
        
        monitoring_thread = threading.Thread(target=monitor_loop, daemon=True)
        monitoring_thread.start()
        logger.info("Background security monitoring started")
    
    def _monitor_threats(self):
        """Monitor for security threats"""
        try:
            # Monitor processes for suspicious activity
            for proc in psutil.process_iter(['pid', 'name', 'cmdline', 'cpu_percent', 'memory_percent']):
                try:
                    proc_info = proc.info
                    cmdline = ' '.join(proc_info['cmdline'] or [])
                    
                    # Check for suspicious command patterns
                    for pattern in self.threat_patterns['suspicious_processes']:
                        if pattern.lower() in cmdline.lower():
                            self._handle_threat_detection({
                                'type': 'suspicious_process',
                                'pid': proc_info['pid'],
                                'name': proc_info['name'],
                                'cmdline': cmdline,
                                'pattern': pattern,
                                'timestamp': datetime.now().isoformat()
                            })
                    
                    # Check for high resource usage
                    if (proc_info['cpu_percent'] and proc_info['cpu_percent'] > self.threat_patterns['high_cpu_threshold']):
                        self._handle_threat_detection({
                            'type': 'high_cpu_usage',
                            'pid': proc_info['pid'],
                            'name': proc_info['name'],
                            'cpu_percent': proc_info['cpu_percent'],
                            'timestamp': datetime.now().isoformat()
                        })
                        
                except (psutil.NoSuchProcess, psutil.AccessDenied):
                    continue
                    
        except Exception as e:
            logger.error(f"Threat monitoring error: {e}")
    
    def _handle_threat_detection(self, threat_info):
        """Handle detected threats with automated response"""
        self.security_alerts.append(threat_info)
        logger.warning(f"Security threat detected: {threat_info}")
        
        if self.auto_response_enabled:
            self._automated_response(threat_info)
    
    def _automated_response(self, threat_info):
        """Automated response to threats"""
        try:
            if threat_info['type'] == 'suspicious_process':
                # Create temporary firewall rule to block the process
                rule_name = f"MCP_BLOCK_{threat_info['pid']}_{int(time.time())}"
                program_path = self._get_process_path(threat_info['pid'])
                
                if program_path:
                    success = self._create_firewall_rule(rule_name, program_path, 'block')
                    if success:
                        # Schedule rule removal
                        self.active_firewall_rules[rule_name] = {
                            'created_at': time.time(),
                            'expires_at': time.time() + self.rule_timeout_seconds,
                            'program_path': program_path,
                            'threat_info': threat_info
                        }
                        logger.info(f"Created temporary firewall rule: {rule_name}")
                        
            elif threat_info['type'] == 'high_cpu_usage':
                # Monitor the high CPU process more closely
                logger.info(f"Monitoring high CPU process: {threat_info['name']} (PID: {threat_info['pid']})")
                
        except Exception as e:
            logger.error(f"Automated response error: {e}")
    
    def _get_process_path(self, pid):
        """Get the executable path of a process"""
        try:
            proc = psutil.Process(pid)
            return proc.exe()
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            return None
    
    def _create_firewall_rule(self, rule_name, program_path, action):
        """Create a Windows firewall rule"""
        try:
            cmd = [
                'netsh', 'advfirewall', 'firewall', 'add', 'rule',
                f'name={rule_name}',
                'dir=out',
                f'action={action}',
                f'program={program_path}',
                'enable=yes'
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, shell=True)
            return result.returncode == 0
            
        except Exception as e:
            logger.error(f"Error creating firewall rule: {e}")
            return False
    
    def _cleanup_expired_firewall_rules(self):
        """Remove expired firewall rules"""
        current_time = time.time()
        expired_rules = []
        
        for rule_name, rule_info in self.active_firewall_rules.items():
            if current_time >= rule_info['expires_at']:
                expired_rules.append(rule_name)
        
        for rule_name in expired_rules:
            try:
                # Remove the firewall rule
                cmd = ['netsh', 'advfirewall', 'firewall', 'delete', 'rule', f'name={rule_name}']
                result = subprocess.run(cmd, capture_output=True, text=True, shell=True)
                
                if result.returncode == 0:
                    logger.info(f"Removed expired firewall rule: {rule_name}")
                    del self.active_firewall_rules[rule_name]
                else:
                    logger.warning(f"Failed to remove firewall rule: {rule_name}")
                    
            except Exception as e:
                logger.error(f"Error removing firewall rule {rule_name}: {e}")

    async def run(self):
        """Run the MCP server"""
        async with stdio_server() as (read_stream, write_stream):
            await self.server.run(
                read_stream,
                write_stream,                InitializationOptions(
                    server_name="windows-security-server",
                    server_version="1.0.0",
                    capabilities=self.server.get_capabilities(
                        notification_options=NotificationOptions(
                            prompts_changed=False,
                            resources_changed=True,
                            tools_changed=False
                        ),
                        experimental_capabilities={},
                    ),
                ),
            )

        # Advanced security tools
        self.server.list_tools = self.list_tools
        self.server.call_tool = self.call_tool
        
        # Register advanced tools
        self.tools.extend([
            Tool(
                name="create_firewall_rule",
                description="Create a temporary Windows firewall rule with auto-expiration",
                inputSchema={
                    "type": "object",
                    "properties": {
                        "program_path": {"type": "string", "description": "Full path to the program"},
                        "action": {"type": "string", "enum": ["allow", "block"], "description": "Action to take"},
                        "duration_minutes": {"type": "number", "default": 60, "description": "Rule duration in minutes"},
                        "rule_name": {"type": "string", "description": "Custom rule name (optional)"}
                    },
                    "required": ["program_path", "action"]
                }
            ),
            Tool(
                name="get_security_alerts",
                description="Get recent security alerts and threat detections",
                inputSchema={
                    "type": "object",
                    "properties": {
                        "limit": {"type": "number", "default": 50, "description": "Maximum number of alerts to return"},
                        "alert_type": {"type": "string", "description": "Filter by alert type (optional)"}
                    }
                }
            ),
            Tool(
                name="manage_auto_response",
                description="Enable/disable automated security responses",
                inputSchema={
                    "type": "object",
                    "properties": {
                        "enabled": {"type": "boolean", "description": "Enable or disable auto-response"},
                        "timeout_minutes": {"type": "number", "description": "Timeout for auto-created rules in minutes"}
                    },
                    "required": ["enabled"]
                }
            ),
            Tool(
                name="scan_network_connections",
                description="Scan for suspicious network connections",
                inputSchema={
                    "type": "object",
                    "properties": {
                        "include_established": {"type": "boolean", "default": True},
                        "include_listening": {"type": "boolean", "default": True}
                    }
                }
            ),
            Tool(
                name="get_active_firewall_rules",
                description="Get currently active temporary firewall rules",
                inputSchema={
                    "type": "object",
                    "properties": {}
                }
            )
        ])

async def main():
    """Main entry point"""
    server = WindowsSecurityServer()
    await server.run()

if __name__ == "__main__":
    asyncio.run(main())
