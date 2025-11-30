import { writable } from 'svelte/store';

interface User {
	id: string;
	name: string;
	email: string;
	avatar?: string;
}

interface AuthState {
	user: User | null;
	loading: boolean;
}

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>({
		user: null,
		loading: true
	});

	return {
		subscribe,
		init: () => {
			const stored = localStorage.getItem('zeteo_user');
			if (stored) {
				set({ user: JSON.parse(stored), loading: false });
			} else {
				set({ user: null, loading: false });
			}
		},
		login: (user: User) => {
			localStorage.setItem('zeteo_user', JSON.stringify(user));
			set({ user, loading: false });
		},
		logout: () => {
			localStorage.removeItem('zeteo_user');
			set({ user: null, loading: false });
		}
	};
}

export const auth = createAuthStore();
