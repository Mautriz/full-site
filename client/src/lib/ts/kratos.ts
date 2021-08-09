class Kratos {
	static BASE_URL = 'http://127.0.0.1:4433';

	private getFlowId() {
		return new URLSearchParams(document.location.search.substring(1)).get('flow');
	}

	getLoginFlow(flowId: string = this.getFlowId()) {
		return fetch(`${Kratos.BASE_URL}/self-service/login/flows?id=${flowId}`, {
			credentials: 'include',
			headers: { accept: 'application/json' }
			// mode: "no-cors"
		});
	}

	getRegistrationFlow(flowId: string = this.getFlowId()) {
		return fetch(`${Kratos.BASE_URL}/self-service/registration/flows?id=${flowId}`, {
			credentials: 'include',
			headers: { accept: 'application/json' }
		});
	}

	getSession() {
		return fetch(`${Kratos.BASE_URL}/sessions/whoami`, {
			credentials: 'include',
			headers: { accept: 'application/json' }
		});
	}

	logout() {
		return fetch(`${Kratos.BASE_URL}/self-service/logout/browser`, {
			credentials: 'include',
			headers: { accept: 'application/json' }
		})
			.then((r) => r.json())
			.then((r) => r.logout_url)
			.then((url) => window.location.assign(url));
	}
}

export const kratos = new Kratos();
