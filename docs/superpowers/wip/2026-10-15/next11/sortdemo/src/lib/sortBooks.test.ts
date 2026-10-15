// Vitest のテスト用関数を読み込む
import { describe, expect, it } from 'vitest'
// テストしたい関数を読み込む
import { sortBooks } from './sortBooks'

// テストで使う3冊ぶんのデータ
const books = [
  { title: 'すいみん学', rating: 3, created_at: '2026-01-10T09:00:00Z' },
  { title: 'あめの本', rating: null, created_at: '2026-03-05T09:00:00Z' },
  { title: 'かぜの歌', rating: 5, created_at: '2026-02-20T09:00:00Z' },
]

describe('sortBooks', () => {
  it('タイトルのあいうえお順に並ぶ', () => {
    const result = sortBooks(books, 'title')
    expect(result.map((b) => b.title)).toEqual(['あめの本', 'かぜの歌', 'すいみん学'])
  })

  it('評価の高い順に並び、評価なしは最後になる', () => {
    const result = sortBooks(books, 'rating')
    expect(result.map((b) => b.rating)).toEqual([5, 3, null])
  })

  it('登録が新しい順に並ぶ', () => {
    const result = sortBooks(books, 'newest')
    expect(result[0].title).toBe('あめの本')
  })

  it('元の配列は書き換えない', () => {
    sortBooks(books, 'title')
    expect(books[0].title).toBe('すいみん学')
  })
})
