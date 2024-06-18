export class HttpClient {

    /**
     * 请求远程服务器
     * @param input string 请求的url路径
     * @param options RequestInit 请求选项
     * @returns Promise<any>
     */
    async request(input: string, options: RequestInit = {}) {
        const headers: Record<string, string> = {
            'Content-Type': 'application/json'
        };

        const token = sessionStorage.getItem('token');
        if (token && token.trim().length > 1) {
            headers['Authorization'] = `Bearer ${token}`;
        }

        options.headers = Object.assign(headers, options.headers);

        if (typeof options.method === 'undefined') {
            options.method = options.body ? 'POST' : 'GET';
        }

        try {
            const response = await fetch(input, options);
            const data = await response.json();
            if (data.Type === 'error') {
                throw new Error(data.Message);
            }
            if (data.Message) {
                postMessage({ message: data.Message, type: 'success' });
            }
            if (data.Stdout) {
                postMessage({ message: data.Stdout, type: 'stdout' });
            }
            if (response.status < 200 || response.status > 400) {
                throw new Error(response.statusText);
            }
            return data.Payload;
        } catch (error) {
            postMessage({ message: error, type: 'danger' });
            throw error;
        }
    }

}
