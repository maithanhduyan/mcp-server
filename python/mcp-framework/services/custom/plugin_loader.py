def plugin_loader():
    """Hàm mẫu cho plugin_loader. Có thể mở rộng để load các plugin động."""
    return []


def load_plugins(plugins_dir):
    """Hàm mẫu cho load_plugins. Có thể mở rộng để load các plugin từ thư mục chỉ định."""
    return []


def get_loaded_services_count():
    """Hàm mẫu trả về số lượng service đã load (mặc định 0)."""
    return 0
