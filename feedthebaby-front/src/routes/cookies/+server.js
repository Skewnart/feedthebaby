
export async function POST({ request, cookies }) {
	const data = await request.formData();
	const token = data.get('token')?.toString() ?? "";

	cookies.set("access_token", token, { path: "/" });

	return new Response(null, { status: 204 });
}