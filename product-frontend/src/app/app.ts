import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Product, ProductService, ApiResponse } from './product.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './app.html',
  styleUrls: ['./app.css']
})
export class AppComponent implements OnInit {
  showChallengeModal = true;
  showCreateModal = false;
  showUpdateModal = false;

  products: Product[] = [];
  newProduct: Product = { name: '', price: 0, description: '', stock: 0 };
  updateProductData: Product = { name: '', price: 0, description: '', stock: 0 };

  alertMessage = '';
  alertColor: 'green' | 'red' = 'green';
  private alertTimeout: any;

  constructor(private productService: ProductService) {}

  ngOnInit(): void {
    this.getAllProducts();
  }

  onChallengeModalClose() {
    this.showChallengeModal = false;
  }

  openCreateModal() {
    this.newProduct = { name: '', price: 0, description: '', stock: 0 };
    this.showCreateModal = true;
  }

  openUpdateModal(product: Product): void {
    this.updateProductData = { ...product };
    this.showUpdateModal = true;
  }

  createProduct(): void {
    this.productService.createProduct(this.newProduct).subscribe({
      next: (response: ApiResponse<number>) => {
        if (response.success) {
          this.showAlert(`Product "${this.newProduct.name}" created with ID: ${response.data}`, true);
          this.newProduct = { name: '', price: 0, description: '', stock: 0 };
          this.showCreateModal = false;
          this.getAllProducts();
        } else {
          this.showAlert(`Failed to create product: ${response.message || 'Unknown error'}`, false);
        }
      },
      error: (err) => {
        this.showAlert(`API Error creating product: ${err.message || 'Could not connect to API'}`, false);
      }
    });
  }

  getAllProducts(): void {
    this.productService.getAllProducts().subscribe({
      next: (response: ApiResponse<Product[]>) => {
        if (response.success) {
          this.products = response.data;
        } else {
          this.showAlert(`Error loading products: ${response.message || 'Unknown error'}`, false);
        }
      },
      error: (err) => {
        this.showAlert(`API Error loading products: ${err.message || 'Could not connect to API'}`, false);
      }
    });
  }

  updateProduct(): void {
    if (this.updateProductData.id === undefined || this.updateProductData.id === null || isNaN(this.updateProductData.id)) {
      this.showAlert('Invalid product for update. Please select a product first.', false);
      return;
    }
    this.productService.updateProduct(this.updateProductData.id, this.updateProductData).subscribe({
      next: (response: ApiResponse<string>) => {
        if (response.success) {
          this.showAlert(`Product ID ${this.updateProductData.id} updated successfully.`, true);
          this.showUpdateModal = false;
          this.getAllProducts();
        } else {
          this.showAlert(`Failed to update product: ${response.message || 'Unknown error'}`, false);
        }
      },
      error: (err) => {
        this.showAlert(`API Error updating product: ${err.message || 'Could not connect to API'}`, false);
      }
    });
  }

  deleteProduct(id: number | undefined): void {
    if (!id) {
      this.showAlert('Invalid product for deletion.', false);
      return;
    }
    if (confirm(`Are you sure you want to delete product with ID ${id}?`)) {
      this.productService.deleteProduct(id).subscribe({
        next: (response: ApiResponse<string>) => {
          if (response.success) {
            this.showAlert(`Product ID ${id} deleted successfully.`, true);
            this.getAllProducts();
          } else {
            this.showAlert(`Failed to delete product: ${response.message || 'Unknown error'}`, false);
          }
        },
        error: (err) => {
          this.showAlert(`API Error deleting product: ${err.message || 'Could not connect to API'}`, false);
        }
      });
    }
  }

  showAlert(message: string, success: boolean = true): void {
    if (this.alertTimeout) {
      clearTimeout(this.alertTimeout);
    }
    this.alertMessage = message;
    this.alertColor = success ? 'green' : 'red';
    this.alertTimeout = setTimeout(() => this.alertMessage = '', 4000);
  }
}
