import { redirect } from '@sveltejs/kit';

export function load({ cookies }) {
    
	const token = cookies.get('access_token');
    if (token)
	    redirect(303, '/');

	return {};
}
