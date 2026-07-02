// PI WEB Desktop API - JavaScript wrapper for Tauri backend
import { invoke } from '@tauri-apps/api/core';

export const PiWebApi = {
    /**
     * Start the PI WEB server
     */
    async startServer() {
        return invoke('start_server');
    },

    /**
     * Stop the PI WEB server
     */
    async stopServer() {
        return invoke('stop_server');
    },

    /**
     * Get the server port
     */
    async getServerPort() {
        return invoke('get_server_port');
    },

    /**
     * Get the PI WEB data directory
     */
    async getDataDir() {
        return invoke('get_data_dir');
    },

    /**
     * Create a directory
     */
    async createDirectory(path) {
        return invoke('create_directory', { path });
    },

    /**
     * List files in a directory
     */
    async listDirectory(path) {
        return invoke('list_directory', { path });
    },

    /**
     * Read a file
     */
    async readFile(path) {
        return invoke('read_file', { path });
    },

    /**
     * Write to a file
     */
    async writeFile(path, content) {
        return invoke('write_file', { path, content });
    },

    /**
     * Execute a shell command
     */
    async executeCommand(command, cwd) {
        return invoke('execute_command', { command, cwd });
    },

    /**
     * Get the current working directory
     */
    async getCwd() {
        return invoke('get_cwd');
    },

    /**
     * Set the current working directory
     */
    async setCwd(path) {
        return invoke('set_cwd', { path });
    },

    /**
     * Get an environment variable
     */
    async getEnvVar(name) {
        return invoke('get_env_var', { name });
    },

    /**
     * Set an environment variable
     */
    async setEnvVar(name, value) {
        return invoke('set_env_var', { name, value });
    },

    /**
     * Get OS info
     */
    async getOsInfo() {
        return invoke('get_os_info');
    },

    /**
     * Get memory info
     */
    async getMemoryInfo() {
        return invoke('get_memory_info');
    },

    /**
     * Get disk usage
     */
    async getDiskUsage(path) {
        return invoke('get_disk_usage', { path });
    },

    /**
     * Open a URL in the default browser
     */
    async openUrl(url) {
        return invoke('open_url', { url });
    },

    /**
     * Show a message box
     */
    async showMessage(title, message) {
        return invoke('show_message', { title, message });
    }
};

export default PiWebApi;
