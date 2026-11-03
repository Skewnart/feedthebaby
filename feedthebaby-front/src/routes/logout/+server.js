import { redirect } from '@sveltejs/kit';

export async function GET({ cookies }) {
	cookies.delete('access_token', { path:'/' });
	redirect(303, '/login');
}