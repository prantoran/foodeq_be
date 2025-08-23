#!/usr/bin/env python3
"""
Simple GUI app for food nutrition analysis.
Upload a food image and get nutritional information from the Rust API.
"""

import tkinter as tk
from tkinter import filedialog, messagebox, scrolledtext
import requests
import base64
import json
from PIL import Image, ImageTk
import io

class NutritionAnalyzerApp:
    def __init__(self, root):
        self.root = root
        self.root.title("Food Nutrition Analyzer")
        self.root.geometry("800x600")
        
        # API endpoint
        self.api_url = "http://localhost:3000/analyze-image"
        
        # Current image data
        self.current_image_base64 = None
        
        self.setup_ui()
    
    def setup_ui(self):
        # Title
        title_label = tk.Label(self.root, text="Food Nutrition Analyzer", 
                              font=("Arial", 16, "bold"))
        title_label.pack(pady=10)
        
        # Upload button
        upload_btn = tk.Button(self.root, text="Upload Food Image", 
                              command=self.upload_image, 
                              font=("Arial", 12),
                              bg="#4CAF50", fg="white",
                              padx=20, pady=10)
        upload_btn.pack(pady=10)
        
        # Image preview frame
        self.image_frame = tk.Frame(self.root, relief=tk.SUNKEN, bd=2)
        self.image_frame.pack(pady=10, padx=20, fill=tk.X)
        
        self.image_label = tk.Label(self.image_frame, text="No image selected", 
                                   font=("Arial", 10), fg="gray")
        self.image_label.pack(pady=20)
        
        # Analyze button
        self.analyze_btn = tk.Button(self.root, text="Analyze Nutrition", 
                                    command=self.analyze_nutrition,
                                    font=("Arial", 12),
                                    bg="#2196F3", fg="white",
                                    padx=20, pady=10,
                                    state=tk.DISABLED)
        self.analyze_btn.pack(pady=10)
        
        # Results area
        results_label = tk.Label(self.root, text="Nutrition Analysis Results:", 
                                font=("Arial", 12, "bold"))
        results_label.pack(pady=(20, 5), anchor=tk.W, padx=20)
        
        self.results_text = scrolledtext.ScrolledText(self.root, 
                                                     height=15, 
                                                     width=80,
                                                     font=("Courier", 10))
        self.results_text.pack(pady=10, padx=20, fill=tk.BOTH, expand=True)
        
        # Status bar
        self.status_var = tk.StringVar()
        self.status_var.set("Ready - Upload an image to get started")
        status_bar = tk.Label(self.root, textvariable=self.status_var, 
                             relief=tk.SUNKEN, anchor=tk.W)
        status_bar.pack(side=tk.BOTTOM, fill=tk.X)
    
    def upload_image(self):
        """Handle image upload and preview."""
        file_path = filedialog.askopenfilename(
            title="Select Food Image",
            filetypes=[
                ("Image files", "*.jpg *.jpeg *.png *.bmp *.gif"),
                ("JPEG files", "*.jpg *.jpeg"),
                ("PNG files", "*.png"),
                ("All files", "*.*")
            ]
        )
        
        if file_path:
            try:
                # Compress and convert image to base64
                self.current_image_base64 = self.compress_and_encode_image(file_path)
                
                # Show preview
                self.show_image_preview(file_path)
                
                # Enable analyze button
                self.analyze_btn.config(state=tk.NORMAL)
                
                self.status_var.set(f"Image loaded and compressed: {file_path.split('/')[-1]}")
                
            except Exception as e:
                messagebox.showerror("Error", f"Failed to load image: {str(e)}")
                self.status_var.set("Error loading image")
    
    def compress_and_encode_image(self, image_path):
        """Compress image and convert to base64 string."""
        try:
            # Open the image
            with Image.open(image_path) as img:
                # Convert to RGB if necessary (handles PNG with transparency, etc.)
                if img.mode in ('RGBA', 'LA', 'P'):
                    # Create white background for transparent images
                    background = Image.new('RGB', img.size, (255, 255, 255))
                    if img.mode == 'P':
                        img = img.convert('RGBA')
                    background.paste(img, mask=img.split()[-1] if img.mode in ('RGBA', 'LA') else None)
                    img = background
                elif img.mode != 'RGB':
                    img = img.convert('RGB')
                
                # Resize image if it's too large (max 256x256 for API efficiency)
                max_size = 256
                if img.width > max_size or img.height > max_size:
                    img.thumbnail((max_size, max_size), Image.Resampling.LANCZOS)
                    print(f"Image resized to: {img.size}")
                
                # Save to bytes buffer with compression
                buffer = io.BytesIO()
                
                # Use JPEG with quality=85 for good compression while maintaining quality
                img.save(buffer, format='JPEG', quality=85, optimize=True)
                
                # Get compressed image data
                compressed_data = buffer.getvalue()
                
                # Calculate compression ratio
                original_size = len(open(image_path, 'rb').read())
                compressed_size = len(compressed_data)
                compression_ratio = (1 - compressed_size / original_size) * 100
                
                print(f"Original size: {original_size:,} bytes")
                print(f"Compressed size: {compressed_size:,} bytes")
                print(f"Compression: {compression_ratio:.1f}% reduction")
                
                # Convert to base64
                return base64.b64encode(compressed_data).decode('utf-8')
                
        except Exception as e:
            print(f"Error compressing image: {e}")
            # Fallback to original method if compression fails
            return self.image_to_base64(image_path)
    
    def image_to_base64(self, image_path):
        """Convert image file to base64 string (fallback method)."""
        with open(image_path, "rb") as image_file:
            return base64.b64encode(image_file.read()).decode('utf-8')
    
    def show_image_preview(self, image_path):
        """Show a preview of the uploaded image."""
        try:
            # Open and resize image for preview
            image = Image.open(image_path)
            
            # Calculate size to fit in preview area (max 300x200)
            max_width, max_height = 300, 200
            image.thumbnail((max_width, max_height), Image.Resampling.LANCZOS)
            
            # Convert to PhotoImage
            photo = ImageTk.PhotoImage(image)
            
            # Update label
            self.image_label.config(image=photo, text="")
            self.image_label.image = photo  # Keep a reference
            
        except Exception as e:
            self.image_label.config(text=f"Preview error: {str(e)}", image="")
    
    def analyze_nutrition(self):
        """Send image to API and display results."""
        if not self.current_image_base64:
            messagebox.showwarning("Warning", "Please upload an image first")
            return
        
        self.status_var.set("Analyzing nutrition... Please wait")
        self.analyze_btn.config(state=tk.DISABLED)
        self.root.update()
        
        try:
            # Prepare request payload
            payload = {
                "image": self.current_image_base64
            }
            
            # Send request to Rust API
            response = requests.post(
                self.api_url,
                json=payload,
                headers={"Content-Type": "application/json"},
                timeout=30
            )
            
            if response.status_code == 200:
                # Parse and display results
                nutrition_data = response.json()
                self.display_results(nutrition_data)
                self.status_var.set("Analysis complete!")
                
            else:
                error_msg = f"API Error: HTTP {response.status_code}\n{response.text}"
                self.results_text.delete(1.0, tk.END)
                self.results_text.insert(tk.END, error_msg)
                self.status_var.set(f"API Error: {response.status_code}")
                
        except requests.exceptions.ConnectionError:
            error_msg = "❌ Connection Error: Could not connect to the API server.\nMake sure your Rust server is running on port 3000."
            self.results_text.delete(1.0, tk.END)
            self.results_text.insert(tk.END, error_msg)
            self.status_var.set("Connection failed")
            
        except requests.exceptions.Timeout:
            error_msg = "❌ Timeout Error: The API request timed out.\nThe server might be taking too long to process the image."
            self.results_text.delete(1.0, tk.END)
            self.results_text.insert(tk.END, error_msg)
            self.status_var.set("Request timed out")
            
        except Exception as e:
            error_msg = f"❌ Error: {str(e)}"
            self.results_text.delete(1.0, tk.END)
            self.results_text.insert(tk.END, error_msg)
            self.status_var.set("Analysis failed")
        
        finally:
            self.analyze_btn.config(state=tk.NORMAL)
    
    def display_results(self, nutrition_data):
        """Display nutrition analysis results in a formatted way."""
        self.results_text.delete(1.0, tk.END)
        
        # Raw JSON response
        self.results_text.insert(tk.END, "=== RAW JSON RESPONSE ===\n")
        self.results_text.insert(tk.END, json.dumps(nutrition_data, indent=2))
        self.results_text.insert(tk.END, "\n\n")
        
        # Formatted results
        if "foods" in nutrition_data and nutrition_data["foods"]:
            self.results_text.insert(tk.END, "=== NUTRITION ANALYSIS ===\n\n")
            
            total_calories = 0
            total_protein = 0
            total_fat = 0
            total_carbs = 0
            
            for i, food in enumerate(nutrition_data["foods"], 1):
                self.results_text.insert(tk.END, f"🍽️  FOOD ITEM #{i}: {food['name']}\n")
                self.results_text.insert(tk.END, f"   Calories: {food['calories']:.1f}\n")
                self.results_text.insert(tk.END, f"   Protein: {food['protein_g']:.1f}g\n")
                self.results_text.insert(tk.END, f"   Fat: {food['fat_g']:.1f}g\n")
                self.results_text.insert(tk.END, f"   Carbohydrates: {food['carbohydrates_g']:.1f}g\n")
                self.results_text.insert(tk.END, f"   Sugar: {food['sugar_g']:.1f}g\n")
                self.results_text.insert(tk.END, f"   Sodium: {food['sodium_mg']:.1f}mg\n")
                self.results_text.insert(tk.END, "\n")
                
                # Add to totals
                total_calories += food['calories']
                total_protein += food['protein_g']
                total_fat += food['fat_g']
                total_carbs += food['carbohydrates_g']
            
            # Summary
            self.results_text.insert(tk.END, "=== SUMMARY ===\n")
            self.results_text.insert(tk.END, f"📊 Total Items: {len(nutrition_data['foods'])}\n")
            self.results_text.insert(tk.END, f"🔥 Total Calories: {total_calories:.1f}\n")
            self.results_text.insert(tk.END, f"💪 Total Protein: {total_protein:.1f}g\n")
            self.results_text.insert(tk.END, f"🥑 Total Fat: {total_fat:.1f}g\n")
            self.results_text.insert(tk.END, f"🍞 Total Carbs: {total_carbs:.1f}g\n")
        else:
            self.results_text.insert(tk.END, "No food items detected in the image.\n")

def main():
    root = tk.Tk()
    app = NutritionAnalyzerApp(root)
    root.mainloop()

if __name__ == "__main__":
    main()