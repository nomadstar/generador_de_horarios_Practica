from django.http import JsonResponse
from django.views.decorators.csrf import ensure_csrf_cookie
from django.views.decorators.http import require_http_methods
from django.shortcuts import render

@ensure_csrf_cookie
@require_http_methods(["GET"])
def get_csrf_token(request):
    return JsonResponse({
        'csrfToken': request.META.get('CSRF_COOKIE'),
        'status': 'success'
    })

def render_react(request):
    return render(request, "index.html")
