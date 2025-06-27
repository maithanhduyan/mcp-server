from fastapi.middleware.cors import CORSMiddleware


def register_middleware(app, settings):
    """Hàm mẫu cho register_middleware. Có thể mở rộng để đăng ký middleware bảo mật."""
    app.add_middleware(
        CORSMiddleware,
        allow_origins=settings.CORS_ORIGINS,
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )
    # Có thể thêm các middleware khác ở đây
    return app
