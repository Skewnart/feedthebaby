import { redirect } from '@sveltejs/kit';

export async function load({ cookies }) {
	const token = cookies.get('access_token');
	if (!token) {
		throw redirect(303, '/login');
	}
	
	return { token };
}
