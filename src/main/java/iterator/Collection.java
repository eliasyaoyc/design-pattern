package iterator;

/** @author Elias (siran0611@gmail.com) */
public interface Collection<E, L> extends Iterable<E> {
  boolean add(E e);

  boolean remove(E e);

  boolean addLink(String key, L i);

  boolean removeLink(String key);

  Iterator<E> iterator();
}
