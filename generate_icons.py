import os
from PIL import Image

def generate_icons(source_path, target_dir):
    try:
        img = Image.open(source_path).convert("RGBA")
        
        # Ensure target dir exists
        os.makedirs(target_dir, exist_ok=True)
        
        # Crop to square if needed (though it should be square)
        w, h = img.size
        if w != h:
            min_dim = min(w, h)
            left = (w - min_dim) / 2
            top = (h - min_dim) / 2
            right = (w + min_dim) / 2
            bottom = (h + min_dim) / 2
            img = img.crop((left, top, right, bottom))
            
        # Create 32x32.png
        img.resize((32, 32), Image.Resampling.LANCZOS).save(os.path.join(target_dir, "32x32.png"))
        
        # Create 128x128.png
        img.resize((128, 128), Image.Resampling.LANCZOS).save(os.path.join(target_dir, "128x128.png"))
        
        # Create icon.png (512x512)
        img.resize((512, 512), Image.Resampling.LANCZOS).save(os.path.join(target_dir, "icon.png"))
        
        # Create icon.ico (multiple sizes)
        icon_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
        img.save(os.path.join(target_dir, "icon.ico"), format="ICO", sizes=icon_sizes)
        
        # Create macOS icon.icns if we can, wait Pillow has IcnsImagePlugin but let's just do .ico for Windows/Linux
        # Tauri usually falls back to icon.png on Linux
        print("Icons generated successfully!")
    except Exception as e:
        print(f"Error generating icons: {e}")

if __name__ == "__main__":
    generate_icons(
        r"C:\Users\vlady\.gemini\antigravity\brain\1adde569-d107-4874-9d22-34903dab3b95\algomentor_logo_1780489337553.png",
        r"src-tauri\icons"
    )
